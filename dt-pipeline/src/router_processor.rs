use dt_common::{
    meta::{dt_data::DtData, row_data::RowData},
};
use dt_connector::rdb_router::RdbRouter;
use crate::processor_chain::DataProcessor;

/// RouterProcessor provides dynamic routing capabilities based on static mappings
/// and content-based rules
pub struct RouterProcessor {
    pub router: RdbRouter,
}

impl RouterProcessor {
    pub fn new(router: RdbRouter) -> Self {
        Self { router }
    }

    /// Process row data and apply routing rules
    /// Content-based routes take precedence over static mappings
    pub fn process(&self, mut row_data: RowData) -> RowData {
        let original_schema = row_data.schema.clone();
        let original_tb = row_data.tb.clone();

        // Try content-based routing first (higher priority)
        if let Some(col_values) = row_data.after.as_ref().or(row_data.before.as_ref()) {
            if let Some((target_schema, target_tb, _topic)) =
                self.router.route_by_content(&original_schema, &original_tb, col_values)
            {
                row_data.schema = target_schema;
                row_data.tb = target_tb;
                // Note: topic routing would be handled by the caller if needed
                return self.apply_column_routing(row_data, &original_schema, &original_tb);
            }
        }

        // Fall back to static routing
        self.router.route_row(row_data)
    }

    /// Process batch of row data
    pub fn process_batch(&self, rows: Vec<RowData>) -> Vec<RowData> {
        rows.into_iter()
            .map(|row| self.process(row))
            .collect()
    }

    /// Apply column-level routing based on original schema/table
    fn apply_column_routing(
        &self,
        mut row_data: RowData,
        original_schema: &str,
        original_tb: &str,
    ) -> RowData {
        if let Some(col_map) = self.router.get_col_map(original_schema, original_tb) {
            if let Some(after) = &mut row_data.after {
                Self::route_columns(after, col_map);
            }
            if let Some(before) = &mut row_data.before {
                Self::route_columns(before, col_map);
            }
        }
        row_data
    }

    fn route_columns(
        col_values: &mut std::collections::HashMap<String, dt_common::meta::col_value::ColValue>,
        col_map: &std::collections::HashMap<String, String>,
    ) {
        let mut new_col_values = std::collections::HashMap::new();
        for (col, col_value) in col_values.drain() {
            let new_col = col_map.get(&col).unwrap_or(&col).clone();
            new_col_values.insert(new_col, col_value);
        }
        *col_values = new_col_values;
    }

    /// Get topic for a given schema and table
    pub fn get_topic(&self, schema: &str, tb: &str) -> &str {
        self.router.get_topic(schema, tb)
    }

    /// Process DtData, applying routing to DML events
    pub fn process_dt_data(&self, dt_data: DtData) -> DtData {
        match dt_data {
            DtData::Dml { row_data } => {
                let routed = self.process(row_data);
                DtData::Dml { row_data: routed }
            }
            DtData::Ddl { ddl_data } => {
                let routed = self.router.route_ddl(ddl_data);
                DtData::Ddl { ddl_data: routed }
            }
            DtData::Struct { struct_data } => {
                let routed = self.router.route_struct(struct_data);
                DtData::Struct { struct_data: routed }
            }
            // Pass through other data types unchanged
            other => other,
        }
    }

    /// Get content-based route for specific data
    /// Returns (schema, table, optional_topic) if a content route matches
    pub fn get_content_route(
        &self,
        schema: &str,
        tb: &str,
        col_values: &std::collections::HashMap<String, dt_common::meta::col_value::ColValue>,
    ) -> Option<(String, String, Option<String>)> {
        self.router.route_by_content(schema, tb, col_values)
    }
}

// Implement DataProcessor trait for processor chaining
impl DataProcessor for RouterProcessor {
    fn process_row(&self, row: RowData) -> Option<RowData> {
        Some(self.process(row))
    }

    fn process_dt_data(&self, dt_data: DtData) -> Option<DtData> {
        Some(RouterProcessor::process_dt_data(self, dt_data))
    }

    fn name(&self) -> &str {
        "RouterProcessor"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dt_common::{
        config::{config_enums::DbType, router_config::{RouterConfig, ContentRoute, RouteRule, RouteCondition, RouteOperator}},
        meta::{col_value::ColValue, row_type::RowType},
    };
    use std::collections::HashMap;

    #[test]
    fn test_static_schema_routing() {
        let config = RouterConfig::Rdb {
            schema_map: "src_db:dst_db".to_string(),
            tb_map: "".to_string(),
            col_map: "".to_string(),
            topic_map: "*.*:default_topic".to_string(),
            content_routes: "".to_string(),
        };

        let router = RdbRouter::from_config(&config, &DbType::Mysql).unwrap();
        let processor = RouterProcessor::new(router);

        let mut after = HashMap::new();
        after.insert("id".to_string(), ColValue::Long(1));

        let row = RowData::new(
            "src_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after),
        );

        let routed = processor.process(row);
        assert_eq!(routed.schema, "dst_db");
        assert_eq!(routed.tb, "users");
    }

    #[test]
    fn test_static_table_routing() {
        let config = RouterConfig::Rdb {
            schema_map: "".to_string(),
            tb_map: "src_db.src_tb:dst_db.dst_tb".to_string(),
            col_map: "".to_string(),
            topic_map: "*.*:default_topic".to_string(),
            content_routes: "".to_string(),
        };

        let router = RdbRouter::from_config(&config, &DbType::Mysql).unwrap();
        let processor = RouterProcessor::new(router);

        let mut after = HashMap::new();
        after.insert("id".to_string(), ColValue::Long(1));

        let row = RowData::new(
            "src_db".to_string(),
            "src_tb".to_string(),
            RowType::Insert,
            None,
            Some(after),
        );

        let routed = processor.process(row);
        assert_eq!(routed.schema, "dst_db");
        assert_eq!(routed.tb, "dst_tb");
    }

    #[test]
    fn test_content_based_routing() {
        let content_route = ContentRoute {
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

        let config = RouterConfig::Rdb {
            schema_map: "".to_string(),
            tb_map: "".to_string(),
            col_map: "".to_string(),
            topic_map: "*.*:default_topic".to_string(),
            content_routes: format!(r#"json:[{}]"#, serde_json::to_string(&content_route).unwrap()),
        };

        let router = RdbRouter::from_config(&config, &DbType::Mysql).unwrap();
        let processor = RouterProcessor::new(router);

        // Test US routing
        let mut after_us = HashMap::new();
        after_us.insert("id".to_string(), ColValue::Long(1));
        after_us.insert("region".to_string(), ColValue::String("us".to_string()));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after_us),
        );

        let routed = processor.process(row);
        assert_eq!(routed.schema, "us_db");
        assert_eq!(routed.tb, "us_users");

        // Test EU routing
        let mut after_eu = HashMap::new();
        after_eu.insert("id".to_string(), ColValue::Long(2));
        after_eu.insert("region".to_string(), ColValue::String("eu".to_string()));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after_eu),
        );

        let routed = processor.process(row);
        assert_eq!(routed.schema, "eu_db");
        assert_eq!(routed.tb, "eu_users");
    }

    #[test]
    fn test_column_routing() {
        let col_map_json = r#"json:[{"db":"test_db","tb":"users","col_map":{"old_name":"new_name","old_email":"new_email"}}]"#;

        let config = RouterConfig::Rdb {
            schema_map: "".to_string(),
            tb_map: "".to_string(),
            col_map: col_map_json.to_string(),
            topic_map: "*.*:default_topic".to_string(),
            content_routes: "".to_string(),
        };

        let router = RdbRouter::from_config(&config, &DbType::Mysql).unwrap();
        let processor = RouterProcessor::new(router);

        let mut after = HashMap::new();
        after.insert("id".to_string(), ColValue::Long(1));
        after.insert("old_name".to_string(), ColValue::String("Alice".to_string()));
        after.insert("old_email".to_string(), ColValue::String("alice@example.com".to_string()));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after),
        );

        let routed = processor.process(row);
        let after = routed.after.unwrap();

        assert!(after.contains_key("id"));
        assert!(after.contains_key("new_name"));
        assert!(after.contains_key("new_email"));
        assert!(!after.contains_key("old_name"));
        assert!(!after.contains_key("old_email"));
    }
}
