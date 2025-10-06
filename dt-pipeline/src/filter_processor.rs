use dt_common::{
    meta::{dt_data::DtData, row_data::RowData},
    rdb_filter::RdbFilter,
};
use crate::processor_chain::DataProcessor;

/// FilterProcessor provides efficient data filtering at database, table, and column levels
/// with support for content-based filtering rules
pub struct FilterProcessor {
    pub filter: RdbFilter,
}

impl FilterProcessor {
    pub fn new(filter: RdbFilter) -> Self {
        Self { filter }
    }

    /// Process row data and filter based on configured rules
    /// Returns None if the row should be filtered out, Some(RowData) otherwise
    pub fn process(&self, row_data: RowData) -> Option<RowData> {
        // Check database/table level filtering
        if self.filter.filter_tb(&row_data.schema, &row_data.tb) {
            return None;
        }

        // Check event type filtering
        if self.filter.filter_event(&row_data.schema, &row_data.tb, &row_data.row_type) {
            return None;
        }

        // Apply column-level filtering (remove ignored columns, keep only do_cols if specified)
        let mut filtered_row = row_data.clone();

        // Filter columns
        if let Some(after) = &mut filtered_row.after {
            Self::filter_columns(after, &self.filter, &row_data.schema, &row_data.tb);
        }
        if let Some(before) = &mut filtered_row.before {
            Self::filter_columns(before, &self.filter, &row_data.schema, &row_data.tb);
        }

        // Apply content-based filtering
        if let Some(col_values) = filtered_row.after.as_ref().or(filtered_row.before.as_ref()) {
            if self.filter.filter_row_content(&row_data.schema, &row_data.tb, col_values) {
                return None;
            }
        }

        Some(filtered_row)
    }

    /// Process batch of row data
    pub fn process_batch(&self, rows: Vec<RowData>) -> Vec<RowData> {
        rows.into_iter()
            .filter_map(|row| self.process(row))
            .collect()
    }

    fn filter_columns(
        col_values: &mut std::collections::HashMap<String, dt_common::meta::col_value::ColValue>,
        filter: &RdbFilter,
        schema: &str,
        tb: &str,
    ) {
        // Remove ignored columns
        if let Some(ignore_cols) = filter.get_ignore_cols(schema, tb) {
            col_values.retain(|col, _| !ignore_cols.contains(col));
        }

        // Keep only do_cols if specified
        if let Some(do_cols) = filter.get_do_cols(schema, tb) {
            col_values.retain(|col, _| do_cols.contains(col));
        }
    }

    /// Filter DtData, supporting different data types
    pub fn process_dt_data(&self, dt_data: DtData) -> Option<DtData> {
        match dt_data {
            DtData::Dml { row_data } => {
                self.process(row_data).map(|filtered| DtData::Dml { row_data: filtered })
            }
            DtData::Ddl { ddl_data } => {
                let (schema, tb) = ddl_data.get_schema_tb();
                if tb.is_empty() {
                    if self.filter.filter_schema(&schema) {
                        None
                    } else {
                        Some(DtData::Ddl { ddl_data })
                    }
                } else if self.filter.filter_tb(&schema, &tb) {
                    None
                } else {
                    Some(DtData::Ddl { ddl_data })
                }
            }
            // Pass through other data types unchanged
            other => Some(other),
        }
    }
}

// Implement DataProcessor trait for processor chaining
impl DataProcessor for FilterProcessor {
    fn process_row(&self, row: RowData) -> Option<RowData> {
        self.process(row)
    }

    fn process_dt_data(&self, dt_data: DtData) -> Option<DtData> {
        FilterProcessor::process_dt_data(self, dt_data)
    }

    fn name(&self) -> &str {
        "FilterProcessor"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dt_common::{
        config::filter_config::{FilterConfig, FilterOperator, ContentFilterRule, TableContentFilter, MatchMode},
        meta::{col_value::ColValue, row_type::RowType},
    };
    use std::collections::HashMap;

    #[test]
    fn test_basic_table_filtering() {
        let config = FilterConfig {
            do_schemas: "test_db".to_string(),
            ignore_tbs: "test_db.ignore_table".to_string(),
            do_events: "insert,update".to_string(),
            ..Default::default()
        };

        let filter = RdbFilter::from_config(&config, &DbType::Mysql).unwrap();
        let processor = FilterProcessor::new(filter);

        // Should filter out ignore_table
        let row = RowData::new(
            "test_db".to_string(),
            "ignore_table".to_string(),
            RowType::Insert,
            None,
            Some(HashMap::new()),
        );
        assert!(processor.process(row).is_none());

        // Should keep allowed_table
        let row = RowData::new(
            "test_db".to_string(),
            "allowed_table".to_string(),
            RowType::Insert,
            None,
            Some(HashMap::new()),
        );
        assert!(processor.process(row).is_some());
    }

    #[test]
    fn test_column_filtering() {
        let config = FilterConfig {
            do_schemas: "test_db".to_string(),
            ignore_cols: r#"json:[{"db":"test_db","tb":"users","ignore_cols":["password","secret"]}]"#.to_string(),
            do_events: "*".to_string(),
            ..Default::default()
        };

        let filter = RdbFilter::from_config(&config, &DbType::Mysql).unwrap();
        let processor = FilterProcessor::new(filter);

        let mut after = HashMap::new();
        after.insert("id".to_string(), ColValue::Long(1));
        after.insert("name".to_string(), ColValue::String("Alice".to_string()));
        after.insert("password".to_string(), ColValue::String("secret123".to_string()));
        after.insert("secret".to_string(), ColValue::String("hidden".to_string()));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after),
        );

        let filtered = processor.process(row).unwrap();
        let after = filtered.after.unwrap();

        assert!(after.contains_key("id"));
        assert!(after.contains_key("name"));
        assert!(!after.contains_key("password"));
        assert!(!after.contains_key("secret"));
    }

    #[test]
    fn test_content_based_filtering() {
        let content_filter = TableContentFilter {
            db: "test_db".to_string(),
            tb: "users".to_string(),
            rules: vec![ContentFilterRule {
                column: "status".to_string(),
                operator: FilterOperator::Eq,
                value: "active".to_string(),
            }],
            match_mode: MatchMode::And,
        };

        let config = FilterConfig {
            do_schemas: "test_db".to_string(),
            do_events: "*".to_string(),
            content_filters: format!(r#"json:[{}]"#, serde_json::to_string(&content_filter).unwrap()),
            ..Default::default()
        };

        let filter = RdbFilter::from_config(&config, &DbType::Mysql).unwrap();
        let processor = FilterProcessor::new(filter);

        // Should keep active user
        let mut after_active = HashMap::new();
        after_active.insert("id".to_string(), ColValue::Long(1));
        after_active.insert("status".to_string(), ColValue::String("active".to_string()));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after_active),
        );
        assert!(processor.process(row).is_some());

        // Should filter out inactive user
        let mut after_inactive = HashMap::new();
        after_inactive.insert("id".to_string(), ColValue::Long(2));
        after_inactive.insert("status".to_string(), ColValue::String("inactive".to_string()));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after_inactive),
        );
        assert!(processor.process(row).is_none());
    }
}
