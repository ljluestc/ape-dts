use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone)]
pub enum RouterConfig {
    Rdb {
        schema_map: String,
        tb_map: String,
        col_map: String,
        topic_map: String,
        // Content-based routing rules
        // Format: json:[{"db":"test_db","tb":"tb_1","routes":[{"condition":{"column":"region","operator":"eq","value":"us"},"target_db":"us_db","target_tb":"users"}]}]
        content_routes: String,
    },
}

/// Content-based routing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRoute {
    pub db: String,
    pub tb: String,
    pub routes: Vec<RouteRule>,
    #[serde(default)]
    pub default_route: Option<DefaultRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRule {
    #[serde(flatten)]
    pub condition: RouteCondition,
    pub target_db: String,
    pub target_tb: String,
    #[serde(default)]
    pub target_topic: Option<String>,
    #[serde(default)]
    pub priority: i32, // Higher priority routes are evaluated first
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RouteCondition {
    Simple {
        column: String,
        operator: RouteOperator,
        value: String,
    },
    Composite {
        conditions: Vec<SimpleCondition>,
        #[serde(default = "default_condition_mode")]
        match_mode: ConditionMode,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleCondition {
    pub column: String,
    pub operator: RouteOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConditionMode {
    And,
    Or,
}

fn default_condition_mode() -> ConditionMode {
    ConditionMode::And
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultRoute {
    pub target_db: String,
    pub target_tb: String,
    #[serde(default)]
    pub target_topic: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RouteOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains,
    Regex,
    In,
    NotIn,
}

impl RouterConfig {
    /// Parse content routes from JSON string
    pub fn parse_content_routes(&self) -> anyhow::Result<Vec<ContentRoute>> {
        match self {
            RouterConfig::Rdb { content_routes, .. } => {
                if content_routes.is_empty() {
                    return Ok(Vec::new());
                }
                let routes: Vec<ContentRoute> = serde_json::from_str(content_routes)?;
                Ok(routes)
            }
        }
    }

    /// Get schema mapping
    pub fn get_schema_map(&self) -> HashMap<String, String> {
        match self {
            RouterConfig::Rdb { schema_map, .. } => {
                Self::parse_mapping(schema_map)
            }
        }
    }

    /// Get table mapping
    pub fn get_tb_map(&self) -> HashMap<String, String> {
        match self {
            RouterConfig::Rdb { tb_map, .. } => {
                Self::parse_mapping(tb_map)
            }
        }
    }

    /// Get column mapping
    pub fn get_col_map(&self) -> HashMap<String, String> {
        match self {
            RouterConfig::Rdb { col_map, .. } => {
                Self::parse_mapping(col_map)
            }
        }
    }

    /// Parse mapping string into HashMap
    /// Format: "source1:target1,source2:target2"
    fn parse_mapping(mapping: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if mapping.is_empty() {
            return map;
        }

        for pair in mapping.split(',') {
            let parts: Vec<&str> = pair.split(':').collect();
            if parts.len() == 2 {
                map.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
            }
        }
        map
    }
}

impl RouteCondition {
    /// Evaluate the route condition against row data
    pub fn evaluate(&self, row_data: &HashMap<String, String>) -> bool {
        match self {
            RouteCondition::Simple { column, operator, value } => {
                if let Some(col_value) = row_data.get(column) {
                    Self::evaluate_operator(operator, col_value, value)
                } else {
                    false
                }
            }
            RouteCondition::Composite { conditions, match_mode } => {
                match match_mode {
                    ConditionMode::And => {
                        conditions.iter().all(|cond| {
                            row_data
                                .get(&cond.column)
                                .map(|v| Self::evaluate_operator(&cond.operator, v, &cond.value))
                                .unwrap_or(false)
                        })
                    }
                    ConditionMode::Or => {
                        conditions.iter().any(|cond| {
                            row_data
                                .get(&cond.column)
                                .map(|v| Self::evaluate_operator(&cond.operator, v, &cond.value))
                                .unwrap_or(false)
                        })
                    }
                }
            }
        }
    }

    fn evaluate_operator(operator: &RouteOperator, col_value: &str, value: &str) -> bool {
        match operator {
            RouteOperator::Eq => col_value == value,
            RouteOperator::Ne => col_value != value,
            RouteOperator::Gt => col_value > value,
            RouteOperator::Gte => col_value >= value,
            RouteOperator::Lt => col_value < value,
            RouteOperator::Lte => col_value <= value,
            RouteOperator::Contains => col_value.contains(value),
            RouteOperator::Regex => {
                if let Ok(re) = regex::Regex::new(value) {
                    re.is_match(col_value)
                } else {
                    false
                }
            }
            RouteOperator::In => {
                let values: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
                values.contains(&col_value)
            }
            RouteOperator::NotIn => {
                let values: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
                !values.contains(&col_value)
            }
        }
    }
}

impl ContentRoute {
    /// Find matching route for the given row data
    pub fn find_route(&self, row_data: &HashMap<String, String>) -> Option<RouteTarget> {
        // Sort routes by priority (higher first)
        let mut sorted_routes = self.routes.clone();
        sorted_routes.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Find first matching route
        for route in sorted_routes {
            if route.condition.evaluate(row_data) {
                return Some(RouteTarget {
                    db: route.target_db.clone(),
                    tb: route.target_tb.clone(),
                    topic: route.target_topic.clone(),
                });
            }
        }

        // Use default route if no match found
        self.default_route.as_ref().map(|dr| RouteTarget {
            db: dr.target_db.clone(),
            tb: dr.target_tb.clone(),
            topic: dr.target_topic.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RouteTarget {
    pub db: String,
    pub tb: String,
    pub topic: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mapping_simple() {
        let mapping = "source1:target1,source2:target2";
        let result = RouterConfig::parse_mapping(mapping);

        assert_eq!(result.len(), 2);
        assert_eq!(result.get("source1"), Some(&"target1".to_string()));
        assert_eq!(result.get("source2"), Some(&"target2".to_string()));
    }

    #[test]
    fn test_parse_mapping_empty() {
        let result = RouterConfig::parse_mapping("");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_mapping_with_spaces() {
        let mapping = " source1 : target1 , source2 : target2 ";
        let result = RouterConfig::parse_mapping(mapping);

        assert_eq!(result.len(), 2);
        assert_eq!(result.get("source1"), Some(&"target1".to_string()));
    }

    #[test]
    fn test_route_condition_simple_eq() {
        let condition = RouteCondition::Simple {
            column: "region".to_string(),
            operator: RouteOperator::Eq,
            value: "us".to_string(),
        };

        let mut data = HashMap::new();
        data.insert("region".to_string(), "us".to_string());
        assert!(condition.evaluate(&data));

        data.insert("region".to_string(), "eu".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_route_condition_simple_contains() {
        let condition = RouteCondition::Simple {
            column: "email".to_string(),
            operator: RouteOperator::Contains,
            value: "@example.com".to_string(),
        };

        let mut data = HashMap::new();
        data.insert("email".to_string(), "user@example.com".to_string());
        assert!(condition.evaluate(&data));

        data.insert("email".to_string(), "user@test.com".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_route_condition_simple_regex() {
        let condition = RouteCondition::Simple {
            column: "id".to_string(),
            operator: RouteOperator::Regex,
            value: r"^[0-9]{3}$".to_string(),
        };

        let mut data = HashMap::new();
        data.insert("id".to_string(), "123".to_string());
        assert!(condition.evaluate(&data));

        data.insert("id".to_string(), "12".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_route_condition_simple_in() {
        let condition = RouteCondition::Simple {
            column: "status".to_string(),
            operator: RouteOperator::In,
            value: "active,pending,processing".to_string(),
        };

        let mut data = HashMap::new();
        data.insert("status".to_string(), "active".to_string());
        assert!(condition.evaluate(&data));

        data.insert("status".to_string(), "inactive".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_route_condition_composite_and() {
        let condition = RouteCondition::Composite {
            conditions: vec![
                SimpleCondition {
                    column: "region".to_string(),
                    operator: RouteOperator::Eq,
                    value: "us".to_string(),
                },
                SimpleCondition {
                    column: "tier".to_string(),
                    operator: RouteOperator::Eq,
                    value: "premium".to_string(),
                },
            ],
            match_mode: ConditionMode::And,
        };

        let mut data = HashMap::new();
        data.insert("region".to_string(), "us".to_string());
        data.insert("tier".to_string(), "premium".to_string());
        assert!(condition.evaluate(&data));

        data.insert("tier".to_string(), "basic".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_route_condition_composite_or() {
        let condition = RouteCondition::Composite {
            conditions: vec![
                SimpleCondition {
                    column: "status".to_string(),
                    operator: RouteOperator::Eq,
                    value: "critical".to_string(),
                },
                SimpleCondition {
                    column: "status".to_string(),
                    operator: RouteOperator::Eq,
                    value: "urgent".to_string(),
                },
            ],
            match_mode: ConditionMode::Or,
        };

        let mut data = HashMap::new();
        data.insert("status".to_string(), "critical".to_string());
        assert!(condition.evaluate(&data));

        data.insert("status".to_string(), "urgent".to_string());
        assert!(condition.evaluate(&data));

        data.insert("status".to_string(), "normal".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_content_route_find_route_simple() {
        let route = ContentRoute {
            db: "test_db".to_string(),
            tb: "users".to_string(),
            routes: vec![
                RouteRule {
                    condition: RouteCondition::Simple {
                        column: "region".to_string(),
                        operator: RouteOperator::Eq,
                        value: "us".to_string(),
                    },
                    target_db: "us_db".to_string(),
                    target_tb: "us_users".to_string(),
                    target_topic: Some("us_topic".to_string()),
                    priority: 0,
                },
                RouteRule {
                    condition: RouteCondition::Simple {
                        column: "region".to_string(),
                        operator: RouteOperator::Eq,
                        value: "eu".to_string(),
                    },
                    target_db: "eu_db".to_string(),
                    target_tb: "eu_users".to_string(),
                    target_topic: Some("eu_topic".to_string()),
                    priority: 0,
                },
            ],
            default_route: None,
        };

        let mut data_us = HashMap::new();
        data_us.insert("region".to_string(), "us".to_string());
        let target = route.find_route(&data_us).unwrap();
        assert_eq!(target.db, "us_db");
        assert_eq!(target.tb, "us_users");

        let mut data_eu = HashMap::new();
        data_eu.insert("region".to_string(), "eu".to_string());
        let target = route.find_route(&data_eu).unwrap();
        assert_eq!(target.db, "eu_db");
    }

    #[test]
    fn test_content_route_priority() {
        let route = ContentRoute {
            db: "test_db".to_string(),
            tb: "users".to_string(),
            routes: vec![
                RouteRule {
                    condition: RouteCondition::Simple {
                        column: "tier".to_string(),
                        operator: RouteOperator::Eq,
                        value: "premium".to_string(),
                    },
                    target_db: "premium_db".to_string(),
                    target_tb: "premium_users".to_string(),
                    target_topic: None,
                    priority: 10,
                },
                RouteRule {
                    condition: RouteCondition::Simple {
                        column: "tier".to_string(),
                        operator: RouteOperator::In,
                        value: "premium,basic".to_string(),
                    },
                    target_db: "all_db".to_string(),
                    target_tb: "all_users".to_string(),
                    target_topic: None,
                    priority: 1,
                },
            ],
            default_route: None,
        };

        let mut data = HashMap::new();
        data.insert("tier".to_string(), "premium".to_string());
        let target = route.find_route(&data).unwrap();
        // Should match higher priority route
        assert_eq!(target.db, "premium_db");
    }

    #[test]
    fn test_content_route_default_route() {
        let route = ContentRoute {
            db: "test_db".to_string(),
            tb: "users".to_string(),
            routes: vec![
                RouteRule {
                    condition: RouteCondition::Simple {
                        column: "region".to_string(),
                        operator: RouteOperator::Eq,
                        value: "us".to_string(),
                    },
                    target_db: "us_db".to_string(),
                    target_tb: "us_users".to_string(),
                    target_topic: None,
                    priority: 0,
                },
            ],
            default_route: Some(DefaultRoute {
                target_db: "default_db".to_string(),
                target_tb: "default_users".to_string(),
                target_topic: Some("default_topic".to_string()),
            }),
        };

        let mut data = HashMap::new();
        data.insert("region".to_string(), "asia".to_string());
        let target = route.find_route(&data).unwrap();
        assert_eq!(target.db, "default_db");
        assert_eq!(target.tb, "default_users");
    }

    #[test]
    fn test_router_config_parse_content_routes() {
        let content_route = ContentRoute {
            db: "test_db".to_string(),
            tb: "users".to_string(),
            routes: vec![],
            default_route: None,
        };

        let config = RouterConfig::Rdb {
            schema_map: "".to_string(),
            tb_map: "".to_string(),
            col_map: "".to_string(),
            topic_map: "".to_string(),
            content_routes: serde_json::to_string(&vec![content_route]).unwrap(),
        };

        let routes = config.parse_content_routes().unwrap();
        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].db, "test_db");
    }

    #[test]
    fn test_router_config_get_schema_map() {
        let config = RouterConfig::Rdb {
            schema_map: "src_db:dst_db,test_db:prod_db".to_string(),
            tb_map: "".to_string(),
            col_map: "".to_string(),
            topic_map: "".to_string(),
            content_routes: "".to_string(),
        };

        let map = config.get_schema_map();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("src_db"), Some(&"dst_db".to_string()));
        assert_eq!(map.get("test_db"), Some(&"prod_db".to_string()));
    }

    #[test]
    fn test_comparison_operators() {
        let mut data = HashMap::new();
        data.insert("age".to_string(), "25".to_string());

        assert!(RouteCondition::evaluate_operator(&RouteOperator::Gt, "25", "20"));
        assert!(RouteCondition::evaluate_operator(&RouteOperator::Gte, "25", "25"));
        assert!(RouteCondition::evaluate_operator(&RouteOperator::Lt, "20", "25"));
        assert!(RouteCondition::evaluate_operator(&RouteOperator::Lte, "25", "25"));
        assert!(RouteCondition::evaluate_operator(&RouteOperator::Ne, "25", "20"));
    }
}
