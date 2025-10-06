use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FilterConfig {
    // Database/Schema level filtering
    pub do_schemas: String,
    pub ignore_schemas: String,
    // Regex pattern for database/schema filtering
    pub do_schemas_regex: String,
    pub ignore_schemas_regex: String,

    // Table level filtering
    pub do_tbs: String,
    pub ignore_tbs: String,
    // Regex pattern for table filtering
    pub do_tbs_regex: String,
    pub ignore_tbs_regex: String,

    // Column level filtering
    pub ignore_cols: String,
    pub do_cols: String, // New: explicitly include specific columns
    // Regex pattern for column filtering
    pub do_cols_regex: String,
    pub ignore_cols_regex: String,

    // Event type filtering
    pub do_events: String,
    pub do_structures: String,
    pub do_ddls: String,
    pub do_dcls: String,
    pub ignore_cmds: String,

    // WHERE clause filtering (table-level conditions)
    pub where_conditions: String,

    // Content-based filtering rules
    // Format: json:[{"db":"test_db","tb":"tb_1","rules":[{"column":"status","operator":"eq","value":"active"}]}]
    pub content_filters: String,
}

/// Content-based filter rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContentFilterRule {
    pub column: String,
    pub operator: FilterOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FilterOperator {
    Eq,      // equals
    Ne,      // not equals
    Gt,      // greater than
    Gte,     // greater than or equal
    Lt,      // less than
    Lte,     // less than or equal
    Contains, // string contains
    Regex,   // regex match
    In,      // in list
    NotIn,   // not in list
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableContentFilter {
    pub db: String,
    pub tb: String,
    pub rules: Vec<ContentFilterRule>,
    #[serde(default = "default_match_mode")]
    pub match_mode: MatchMode, // AND or OR combination
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MatchMode {
    And,
    Or,
}

fn default_match_mode() -> MatchMode {
    MatchMode::And
}

impl FilterConfig {
    /// Parse content filters from JSON string
    pub fn parse_content_filters(&self) -> anyhow::Result<Vec<TableContentFilter>> {
        if self.content_filters.is_empty() {
            return Ok(Vec::new());
        }

        let filters: Vec<TableContentFilter> = serde_json::from_str(&self.content_filters)?;
        Ok(filters)
    }

    /// Check if a database/schema should be included based on filtering rules
    pub fn should_include_schema(&self, schema: &str) -> bool {
        // Check regex patterns first
        if !self.do_schemas_regex.is_empty() {
            if let Ok(re) = regex::Regex::new(&self.do_schemas_regex) {
                if !re.is_match(schema) {
                    return false;
                }
            }
        }

        if !self.ignore_schemas_regex.is_empty() {
            if let Ok(re) = regex::Regex::new(&self.ignore_schemas_regex) {
                if re.is_match(schema) {
                    return false;
                }
            }
        }

        // Check explicit lists
        if !self.ignore_schemas.is_empty() {
            let ignore_list: Vec<&str> = self.ignore_schemas.split(',').map(|s| s.trim()).collect();
            if ignore_list.contains(&schema) {
                return false;
            }
        }

        if !self.do_schemas.is_empty() {
            let do_list: Vec<&str> = self.do_schemas.split(',').map(|s| s.trim()).collect();
            return do_list.contains(&schema);
        }

        true
    }

    /// Check if a table should be included based on filtering rules
    pub fn should_include_table(&self, schema: &str, table: &str) -> bool {
        let full_name = format!("{}.{}", schema, table);

        // Check regex patterns first
        if !self.do_tbs_regex.is_empty() {
            if let Ok(re) = regex::Regex::new(&self.do_tbs_regex) {
                if !re.is_match(&full_name) && !re.is_match(table) {
                    return false;
                }
            }
        }

        if !self.ignore_tbs_regex.is_empty() {
            if let Ok(re) = regex::Regex::new(&self.ignore_tbs_regex) {
                if re.is_match(&full_name) || re.is_match(table) {
                    return false;
                }
            }
        }

        // Check explicit lists
        if !self.ignore_tbs.is_empty() {
            let ignore_list: Vec<&str> = self.ignore_tbs.split(',').map(|s| s.trim()).collect();
            if ignore_list.contains(&full_name.as_str()) || ignore_list.contains(&table) {
                return false;
            }
        }

        if !self.do_tbs.is_empty() {
            let do_list: Vec<&str> = self.do_tbs.split(',').map(|s| s.trim()).collect();
            return do_list.contains(&full_name.as_str()) || do_list.contains(&table);
        }

        true
    }

    /// Check if a column should be included based on filtering rules
    pub fn should_include_column(&self, schema: &str, table: &str, column: &str) -> bool {
        let full_name = format!("{}.{}.{}", schema, table, column);
        let tb_col = format!("{}.{}", table, column);

        // Check regex patterns first
        if !self.do_cols_regex.is_empty() {
            if let Ok(re) = regex::Regex::new(&self.do_cols_regex) {
                if !re.is_match(&full_name) && !re.is_match(&tb_col) && !re.is_match(column) {
                    return false;
                }
            }
        }

        if !self.ignore_cols_regex.is_empty() {
            if let Ok(re) = regex::Regex::new(&self.ignore_cols_regex) {
                if re.is_match(&full_name) || re.is_match(&tb_col) || re.is_match(column) {
                    return false;
                }
            }
        }

        // Check explicit lists
        if !self.ignore_cols.is_empty() {
            let ignore_list: Vec<&str> = self.ignore_cols.split(',').map(|s| s.trim()).collect();
            if ignore_list.contains(&full_name.as_str())
                || ignore_list.contains(&tb_col.as_str())
                || ignore_list.contains(&column) {
                return false;
            }
        }

        if !self.do_cols.is_empty() {
            let do_list: Vec<&str> = self.do_cols.split(',').map(|s| s.trim()).collect();
            return do_list.contains(&full_name.as_str())
                || do_list.contains(&tb_col.as_str())
                || do_list.contains(&column);
        }

        true
    }
}

impl ContentFilterRule {
    /// Evaluate the filter rule against a column value
    pub fn evaluate(&self, value: &str) -> bool {
        match &self.operator {
            FilterOperator::Eq => value == self.value,
            FilterOperator::Ne => value != self.value,
            FilterOperator::Gt => value > self.value.as_str(),
            FilterOperator::Gte => value >= self.value.as_str(),
            FilterOperator::Lt => value < self.value.as_str(),
            FilterOperator::Lte => value <= self.value.as_str(),
            FilterOperator::Contains => value.contains(&self.value),
            FilterOperator::Regex => {
                if let Ok(re) = regex::Regex::new(&self.value) {
                    re.is_match(value)
                } else {
                    false
                }
            }
            FilterOperator::In => {
                let values: Vec<&str> = self.value.split(',').map(|s| s.trim()).collect();
                values.contains(&value)
            }
            FilterOperator::NotIn => {
                let values: Vec<&str> = self.value.split(',').map(|s| s.trim()).collect();
                !values.contains(&value)
            }
        }
    }
}

impl TableContentFilter {
    /// Check if all rules match for the given row data
    pub fn matches(&self, row_data: &std::collections::HashMap<String, String>) -> bool {
        if self.rules.is_empty() {
            return true;
        }

        match self.match_mode {
            MatchMode::And => {
                self.rules.iter().all(|rule| {
                    row_data
                        .get(&rule.column)
                        .map(|v| rule.evaluate(v))
                        .unwrap_or(false)
                })
            }
            MatchMode::Or => {
                self.rules.iter().any(|rule| {
                    row_data
                        .get(&rule.column)
                        .map(|v| rule.evaluate(v))
                        .unwrap_or(false)
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_filtering_with_do_list() {
        let mut config = FilterConfig::default();
        config.do_schemas = "db1,db2,db3".to_string();

        assert!(config.should_include_schema("db1"));
        assert!(config.should_include_schema("db2"));
        assert!(!config.should_include_schema("db4"));
    }

    #[test]
    fn test_schema_filtering_with_ignore_list() {
        let mut config = FilterConfig::default();
        config.ignore_schemas = "test_db,temp_db".to_string();

        assert!(!config.should_include_schema("test_db"));
        assert!(!config.should_include_schema("temp_db"));
        assert!(config.should_include_schema("prod_db"));
    }

    #[test]
    fn test_schema_filtering_with_regex() {
        let mut config = FilterConfig::default();
        config.do_schemas_regex = "^prod_.*".to_string();

        assert!(config.should_include_schema("prod_db"));
        assert!(config.should_include_schema("prod_analytics"));
        assert!(!config.should_include_schema("test_db"));
    }

    #[test]
    fn test_schema_filtering_ignore_regex() {
        let mut config = FilterConfig::default();
        config.ignore_schemas_regex = "^(test|tmp)_.*".to_string();

        assert!(!config.should_include_schema("test_db"));
        assert!(!config.should_include_schema("tmp_data"));
        assert!(config.should_include_schema("prod_db"));
    }

    #[test]
    fn test_table_filtering_with_full_name() {
        let mut config = FilterConfig::default();
        config.do_tbs = "test_db.users,test_db.orders".to_string();

        assert!(config.should_include_table("test_db", "users"));
        assert!(config.should_include_table("test_db", "orders"));
        assert!(!config.should_include_table("test_db", "logs"));
    }

    #[test]
    fn test_table_filtering_with_regex() {
        let mut config = FilterConfig::default();
        config.do_tbs_regex = ".*_data$".to_string();

        assert!(config.should_include_table("test_db", "user_data"));
        assert!(config.should_include_table("test_db", "order_data"));
        assert!(!config.should_include_table("test_db", "users"));
    }

    #[test]
    fn test_column_filtering_basic() {
        let mut config = FilterConfig::default();
        config.ignore_cols = "password,secret_key".to_string();

        assert!(!config.should_include_column("db", "users", "password"));
        assert!(!config.should_include_column("db", "users", "secret_key"));
        assert!(config.should_include_column("db", "users", "username"));
    }

    #[test]
    fn test_column_filtering_with_table_prefix() {
        let mut config = FilterConfig::default();
        config.ignore_cols = "users.password,users.email".to_string();

        assert!(!config.should_include_column("db", "users", "password"));
        assert!(!config.should_include_column("db", "users", "email"));
        assert!(config.should_include_column("db", "users", "username"));
        assert!(config.should_include_column("db", "orders", "password")); // different table
    }

    #[test]
    fn test_column_filtering_with_regex() {
        let mut config = FilterConfig::default();
        config.ignore_cols_regex = "^(password|secret).*".to_string();

        assert!(!config.should_include_column("db", "users", "password"));
        assert!(!config.should_include_column("db", "users", "password_hash"));
        assert!(!config.should_include_column("db", "users", "secret_key"));
        assert!(config.should_include_column("db", "users", "username"));
    }

    #[test]
    fn test_content_filter_rule_eq() {
        let rule = ContentFilterRule {
            column: "status".to_string(),
            operator: FilterOperator::Eq,
            value: "active".to_string(),
        };

        assert!(rule.evaluate("active"));
        assert!(!rule.evaluate("inactive"));
    }

    #[test]
    fn test_content_filter_rule_contains() {
        let rule = ContentFilterRule {
            column: "email".to_string(),
            operator: FilterOperator::Contains,
            value: "@example.com".to_string(),
        };

        assert!(rule.evaluate("user@example.com"));
        assert!(!rule.evaluate("user@test.com"));
    }

    #[test]
    fn test_content_filter_rule_regex() {
        let rule = ContentFilterRule {
            column: "email".to_string(),
            operator: FilterOperator::Regex,
            value: r"^[a-z]+@example\.com$".to_string(),
        };

        assert!(rule.evaluate("user@example.com"));
        assert!(!rule.evaluate("User123@example.com"));
    }

    #[test]
    fn test_content_filter_rule_in() {
        let rule = ContentFilterRule {
            column: "role".to_string(),
            operator: FilterOperator::In,
            value: "admin,moderator,editor".to_string(),
        };

        assert!(rule.evaluate("admin"));
        assert!(rule.evaluate("moderator"));
        assert!(!rule.evaluate("user"));
    }

    #[test]
    fn test_content_filter_rule_comparison() {
        let rule = ContentFilterRule {
            column: "age".to_string(),
            operator: FilterOperator::Gte,
            value: "18".to_string(),
        };

        assert!(rule.evaluate("25"));
        assert!(rule.evaluate("18"));
        assert!(!rule.evaluate("15"));
    }

    #[test]
    fn test_table_content_filter_and_mode() {
        let filter = TableContentFilter {
            db: "test_db".to_string(),
            tb: "users".to_string(),
            rules: vec![
                ContentFilterRule {
                    column: "status".to_string(),
                    operator: FilterOperator::Eq,
                    value: "active".to_string(),
                },
                ContentFilterRule {
                    column: "verified".to_string(),
                    operator: FilterOperator::Eq,
                    value: "true".to_string(),
                },
            ],
            match_mode: MatchMode::And,
        };

        let mut row1 = std::collections::HashMap::new();
        row1.insert("status".to_string(), "active".to_string());
        row1.insert("verified".to_string(), "true".to_string());
        assert!(filter.matches(&row1));

        let mut row2 = std::collections::HashMap::new();
        row2.insert("status".to_string(), "active".to_string());
        row2.insert("verified".to_string(), "false".to_string());
        assert!(!filter.matches(&row2));
    }

    #[test]
    fn test_table_content_filter_or_mode() {
        let filter = TableContentFilter {
            db: "test_db".to_string(),
            tb: "users".to_string(),
            rules: vec![
                ContentFilterRule {
                    column: "role".to_string(),
                    operator: FilterOperator::Eq,
                    value: "admin".to_string(),
                },
                ContentFilterRule {
                    column: "role".to_string(),
                    operator: FilterOperator::Eq,
                    value: "moderator".to_string(),
                },
            ],
            match_mode: MatchMode::Or,
        };

        let mut row1 = std::collections::HashMap::new();
        row1.insert("role".to_string(), "admin".to_string());
        assert!(filter.matches(&row1));

        let mut row2 = std::collections::HashMap::new();
        row2.insert("role".to_string(), "moderator".to_string());
        assert!(filter.matches(&row2));

        let mut row3 = std::collections::HashMap::new();
        row3.insert("role".to_string(), "user".to_string());
        assert!(!filter.matches(&row3));
    }

    #[test]
    fn test_parse_content_filters() {
        let mut config = FilterConfig::default();
        config.content_filters = r#"[
            {
                "db": "test_db",
                "tb": "users",
                "rules": [
                    {
                        "column": "status",
                        "operator": "eq",
                        "value": "active"
                    }
                ],
                "match_mode": "and"
            }
        ]"#.to_string();

        let filters = config.parse_content_filters().unwrap();
        assert_eq!(filters.len(), 1);
        assert_eq!(filters[0].db, "test_db");
        assert_eq!(filters[0].tb, "users");
        assert_eq!(filters[0].rules.len(), 1);
    }

    #[test]
    fn test_combined_schema_and_table_filtering() {
        let mut config = FilterConfig::default();
        config.do_schemas = "prod_db".to_string();
        config.ignore_tbs = "prod_db.logs".to_string();

        assert!(config.should_include_schema("prod_db"));
        assert!(!config.should_include_schema("test_db"));
        assert!(config.should_include_table("prod_db", "users"));
        assert!(!config.should_include_table("prod_db", "logs"));
    }
}
