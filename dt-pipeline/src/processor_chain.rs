use dt_common::meta::{dt_data::DtData, row_data::RowData};

/// Trait for data processors that can be chained together
pub trait DataProcessor: Send + Sync {
    /// Process a single row of data
    /// Returns None if the row should be filtered out
    fn process_row(&self, row: RowData) -> Option<RowData>;

    /// Process DtData (supports DML, DDL, etc.)
    /// Returns None if the data should be filtered out
    fn process_dt_data(&self, dt_data: DtData) -> Option<DtData>;

    /// Get processor name for logging/debugging
    fn name(&self) -> &str;
}

/// Chain of processors that can be applied sequentially to data
pub struct ProcessorChain {
    processors: Vec<Box<dyn DataProcessor>>,
}

impl ProcessorChain {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    /// Add a processor to the end of the chain
    pub fn add_processor(&mut self, processor: Box<dyn DataProcessor>) -> &mut Self {
        self.processors.push(processor);
        self
    }

    /// Process a row through all processors in the chain
    /// Returns None if any processor filters out the row
    pub fn process_row(&self, mut row: RowData) -> Option<RowData> {
        for processor in &self.processors {
            row = processor.process_row(row)?;
        }
        Some(row)
    }

    /// Process DtData through all processors in the chain
    /// Returns None if any processor filters out the data
    pub fn process_dt_data(&self, mut dt_data: DtData) -> Option<DtData> {
        for processor in &self.processors {
            dt_data = processor.process_dt_data(dt_data)?;
        }
        Some(dt_data)
    }

    /// Process a batch of rows through the chain
    pub fn process_batch(&self, rows: Vec<RowData>) -> Vec<RowData> {
        rows.into_iter()
            .filter_map(|row| self.process_row(row))
            .collect()
    }

    /// Process a batch of DtData through the chain
    pub fn process_dt_data_batch(&self, items: Vec<DtData>) -> Vec<DtData> {
        items
            .into_iter()
            .filter_map(|item| self.process_dt_data(item))
            .collect()
    }

    /// Get the number of processors in the chain
    pub fn len(&self) -> usize {
        self.processors.len()
    }

    /// Check if the chain is empty
    pub fn is_empty(&self) -> bool {
        self.processors.is_empty()
    }

    /// Get processor names for debugging
    pub fn processor_names(&self) -> Vec<&str> {
        self.processors.iter().map(|p| p.name()).collect()
    }
}

impl Default for ProcessorChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dt_common::meta::{col_value::ColValue, row_type::RowType};
    use std::collections::HashMap;

    // Mock processor that filters rows where id < 10
    struct IdFilterProcessor;

    impl DataProcessor for IdFilterProcessor {
        fn process_row(&self, row: RowData) -> Option<RowData> {
            if let Some(after) = &row.after {
                if let Some(ColValue::Long(id)) = after.get("id") {
                    if *id < 10 {
                        return None;
                    }
                }
            }
            Some(row)
        }

        fn process_dt_data(&self, dt_data: DtData) -> Option<DtData> {
            match dt_data {
                DtData::Dml { row_data } => {
                    self.process_row(row_data).map(|r| DtData::Dml { row_data: r })
                }
                other => Some(other),
            }
        }

        fn name(&self) -> &str {
            "IdFilterProcessor"
        }
    }

    // Mock processor that adds a processed flag
    struct FlagAddingProcessor;

    impl DataProcessor for FlagAddingProcessor {
        fn process_row(&self, mut row: RowData) -> Option<RowData> {
            if let Some(after) = &mut row.after {
                after.insert("processed".to_string(), ColValue::String("true".to_string()));
            }
            Some(row)
        }

        fn process_dt_data(&self, dt_data: DtData) -> Option<DtData> {
            match dt_data {
                DtData::Dml { row_data } => {
                    self.process_row(row_data).map(|r| DtData::Dml { row_data: r })
                }
                other => Some(other),
            }
        }

        fn name(&self) -> &str {
            "FlagAddingProcessor"
        }
    }

    #[test]
    fn test_empty_chain() {
        let chain = ProcessorChain::new();
        assert!(chain.is_empty());
        assert_eq!(chain.len(), 0);

        let mut after = HashMap::new();
        after.insert("id".to_string(), ColValue::Long(5));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after.clone()),
        );

        let result = chain.process_row(row);
        assert!(result.is_some());
    }

    #[test]
    fn test_single_processor() {
        let mut chain = ProcessorChain::new();
        chain.add_processor(Box::new(IdFilterProcessor));

        assert_eq!(chain.len(), 1);
        assert_eq!(chain.processor_names(), vec!["IdFilterProcessor"]);

        // Should filter out id < 10
        let mut after = HashMap::new();
        after.insert("id".to_string(), ColValue::Long(5));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after),
        );

        assert!(chain.process_row(row).is_none());

        // Should keep id >= 10
        let mut after = HashMap::new();
        after.insert("id".to_string(), ColValue::Long(15));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after),
        );

        assert!(chain.process_row(row).is_some());
    }

    #[test]
    fn test_chained_processors() {
        let mut chain = ProcessorChain::new();
        chain
            .add_processor(Box::new(IdFilterProcessor))
            .add_processor(Box::new(FlagAddingProcessor));

        assert_eq!(chain.len(), 2);
        assert_eq!(
            chain.processor_names(),
            vec!["IdFilterProcessor", "FlagAddingProcessor"]
        );

        // Should filter out id < 10
        let mut after = HashMap::new();
        after.insert("id".to_string(), ColValue::Long(5));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after),
        );

        assert!(chain.process_row(row).is_none());

        // Should keep id >= 10 and add processed flag
        let mut after = HashMap::new();
        after.insert("id".to_string(), ColValue::Long(15));

        let row = RowData::new(
            "test_db".to_string(),
            "users".to_string(),
            RowType::Insert,
            None,
            Some(after),
        );

        let result = chain.process_row(row).unwrap();
        let after = result.after.unwrap();
        assert_eq!(after.get("id"), Some(&ColValue::Long(15)));
        assert_eq!(
            after.get("processed"),
            Some(&ColValue::String("true".to_string()))
        );
    }

    #[test]
    fn test_batch_processing() {
        let mut chain = ProcessorChain::new();
        chain.add_processor(Box::new(IdFilterProcessor));

        let rows: Vec<RowData> = (0..20)
            .map(|i| {
                let mut after = HashMap::new();
                after.insert("id".to_string(), ColValue::Long(i));
                RowData::new(
                    "test_db".to_string(),
                    "users".to_string(),
                    RowType::Insert,
                    None,
                    Some(after),
                )
            })
            .collect();

        let result = chain.process_batch(rows);
        // Should only keep rows with id >= 10
        assert_eq!(result.len(), 10);
        for row in result {
            if let Some(after) = row.after {
                if let Some(ColValue::Long(id)) = after.get("id") {
                    assert!(*id >= 10);
                }
            }
        }
    }
}
