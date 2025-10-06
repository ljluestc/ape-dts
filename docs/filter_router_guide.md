# Advanced Data Filtering and Routing System

This document describes the comprehensive filtering and routing capabilities in ape-dts.

## Overview

The system provides three levels of data processing:

1. **FilterProcessor** - Filters data at database, table, column, and content levels
2. **RouterProcessor** - Routes data based on static mappings and content-based rules
3. **ProcessorChain** - Chains multiple processors for complex data pipelines

## Filter Configuration

### Database/Schema Level Filtering

Filter which databases/schemas to include or exclude:

```ini
# Include specific databases
do_schemas=db1,db2,db_prod*

# Exclude specific databases
ignore_schemas=db_test*,temp_db
```

### Table Level Filtering

Filter specific tables with wildcard support:

```ini
# Include specific tables
do_tbs=db1.users,db1.orders,db2.*

# Exclude specific tables
ignore_tbs=*.temp_*,*.backup_*
```

### Event Type Filtering

Filter by operation type:

```ini
# Only process insert and update events
do_events=insert,update

# Process all DDL types
do_ddls=*

# Process specific DDL types
do_ddls=create_table,alter_table
```

### Column Level Filtering

#### Ignore Columns

Remove sensitive columns from data:

```ini
ignore_cols=json:[
  {"db":"user_db","tb":"users","ignore_cols":["password","ssn","credit_card"]},
  {"db":"logs_db","tb":"audit_log","ignore_cols":["internal_notes"]}
]
```

#### Include Only Specific Columns

Keep only specific columns (whitelist approach):

```ini
do_cols=json:[
  {"db":"user_db","tb":"users","do_cols":["id","username","email","created_at"]},
  {"db":"product_db","tb":"products","do_cols":["id","name","price"]}
]
```

### Content-Based Filtering

Filter rows based on column values:

```ini
content_filters=json:[
  {
    "db": "sales_db",
    "tb": "orders",
    "rules": [
      {"column": "status", "operator": "eq", "value": "completed"},
      {"column": "amount", "operator": "gte", "value": "100"}
    ],
    "match_mode": "and"
  },
  {
    "db": "user_db",
    "tb": "users",
    "rules": [
      {"column": "status", "operator": "in", "value": "active,premium,vip"}
    ],
    "match_mode": "and"
  }
]
```

**Supported Operators:**
- `eq` - equals
- `ne` - not equals
- `gt` - greater than
- `gte` - greater than or equal
- `lt` - less than
- `lte` - less than or equal
- `contains` - string contains substring
- `regex` - regex pattern match
- `in` - value in comma-separated list
- `notin` - value not in comma-separated list

**Match Modes:**
- `and` - All rules must match
- `or` - Any rule must match

### WHERE Clause Filtering

SQL WHERE conditions for fine-grained filtering:

```ini
where_conditions=json:[
  {"db":"sales_db","tb":"orders","condition":"created_at > '2024-01-01' AND total_amount > 1000"},
  {"db":"user_db","tb":"users","condition":"last_login > NOW() - INTERVAL 30 DAY"}
]
```

## Router Configuration

### Schema/Database Routing

Map source databases to target databases:

```ini
# Route src_db to dest_db
schema_map=src_db:dest_db,legacy_db:new_db
```

### Table Routing

Map specific tables to different names:

```ini
# Route src_db.old_users to dest_db.users
tb_map=src_db.old_users:dest_db.users,src_db.orders:dest_db.customer_orders
```

### Column Routing

Rename columns during routing:

```ini
col_map=json:[
  {
    "db":"user_db",
    "tb":"users",
    "col_map":{
      "old_email":"email",
      "old_username":"username",
      "old_created":"created_at"
    }
  }
]
```

### Topic Routing (for Kafka/messaging)

Route data to different topics:

```ini
# Default topic
topic_map=*.*:default_topic

# Specific routing
topic_map=user_db.*:user_events,order_db.*:order_events,order_db.high_value:priority_orders
```

### Content-Based Routing

Route data to different destinations based on content:

```ini
content_routes=json:[
  {
    "db": "global_users",
    "tb": "users",
    "routes": [
      {
        "condition": {"column": "region", "operator": "eq", "value": "us"},
        "target_db": "us_users_db",
        "target_tb": "users",
        "target_topic": "us_user_events"
      },
      {
        "condition": {"column": "region", "operator": "eq", "value": "eu"},
        "target_db": "eu_users_db",
        "target_tb": "users",
        "target_topic": "eu_user_events"
      },
      {
        "condition": {"column": "region", "operator": "eq", "value": "asia"},
        "target_db": "asia_users_db",
        "target_tb": "users",
        "target_topic": "asia_user_events"
      }
    ]
  },
  {
    "db": "orders",
    "tb": "transactions",
    "routes": [
      {
        "condition": {"column": "amount", "operator": "gte", "value": "10000"},
        "target_db": "high_value_orders",
        "target_tb": "transactions",
        "target_topic": "high_value_alerts"
      }
    ]
  }
]
```

## Usage Examples

### Example 1: Basic Filtering and Routing

```rust
use dt_common::{
    config::{
        config_enums::DbType,
        filter_config::FilterConfig,
        router_config::RouterConfig,
    },
    rdb_filter::RdbFilter,
};
use dt_connector::rdb_router::RdbRouter;
use dt_pipeline::{
    filter_processor::FilterProcessor,
    router_processor::RouterProcessor,
};

// Create filter
let filter_config = FilterConfig {
    do_schemas: "production_db".to_string(),
    ignore_tbs: "production_db.temp_*".to_string(),
    ignore_cols: r#"json:[{"db":"production_db","tb":"users","ignore_cols":["password"]}]"#.to_string(),
    do_events: "insert,update,delete".to_string(),
    ..Default::default()
};

let filter = RdbFilter::from_config(&filter_config, &DbType::Mysql)?;
let filter_processor = FilterProcessor::new(filter);

// Create router
let router_config = RouterConfig::Rdb {
    schema_map: "production_db:replicated_db".to_string(),
    tb_map: "".to_string(),
    col_map: "".to_string(),
    topic_map: "*.*:data_replication".to_string(),
    content_routes: "".to_string(),
};

let router = RdbRouter::from_config(&router_config, &DbType::Mysql)?;
let router_processor = RouterProcessor::new(router);

// Process data
if let Some(filtered_row) = filter_processor.process(row_data) {
    let routed_row = router_processor.process(filtered_row);
    // Send routed_row to destination
}
```

### Example 2: Processor Chain

```rust
use dt_pipeline::processor_chain::{DataProcessor, ProcessorChain};

// Create processor chain
let mut chain = ProcessorChain::new();
chain
    .add_processor(Box::new(FilterProcessorAdapter::new(filter_processor)))
    .add_processor(Box::new(RouterProcessorAdapter::new(router_processor)));

// Process batch
let processed_rows = chain.process_batch(rows);
```

### Example 3: Multi-Region Data Distribution

```ini
[filter]
do_schemas=global_users
do_events=*

# Only sync active users
content_filters=json:[
  {
    "db": "global_users",
    "tb": "users",
    "rules": [
      {"column": "status", "operator": "eq", "value": "active"}
    ],
    "match_mode": "and"
  }
]

[router]
# Route users to regional databases based on region column
content_routes=json:[
  {
    "db": "global_users",
    "tb": "users",
    "routes": [
      {
        "condition": {"column": "region", "operator": "eq", "value": "north_america"},
        "target_db": "na_users",
        "target_tb": "users",
        "target_topic": "na_user_events"
      },
      {
        "condition": {"column": "region", "operator": "eq", "value": "europe"},
        "target_db": "eu_users",
        "target_tb": "users",
        "target_topic": "eu_user_events"
      }
    ]
  }
]
```

### Example 4: PII Filtering with Column Masking

```ini
[filter]
do_schemas=customer_db
do_events=*

# Remove PII columns
ignore_cols=json:[
  {
    "db":"customer_db",
    "tb":"customers",
    "ignore_cols":["ssn","credit_card","drivers_license","passport_number"]
  },
  {
    "db":"customer_db",
    "tb":"payment_methods",
    "ignore_cols":["card_number","cvv","bank_account"]
  }
]

# Only sync certain data
do_cols=json:[
  {
    "db":"customer_db",
    "tb":"customers",
    "do_cols":["id","name","email","created_at","country"]
  }
]
```

### Example 5: High-Value Transaction Routing

```ini
[filter]
do_schemas=transactions_db
do_events=insert

[router]
# Route high-value transactions to special processing
content_routes=json:[
  {
    "db": "transactions_db",
    "tb": "payments",
    "routes": [
      {
        "condition": {"column": "amount", "operator": "gte", "value": "10000"},
        "target_db": "fraud_detection_db",
        "target_tb": "high_value_payments",
        "target_topic": "fraud_detection_queue"
      },
      {
        "condition": {"column": "currency", "operator": "eq", "value": "BTC"},
        "target_db": "crypto_transactions",
        "target_tb": "bitcoin_payments",
        "target_topic": "crypto_monitoring"
      }
    ]
  }
]
```

## Performance Considerations

1. **Caching**: The filter system uses DashMap for thread-safe caching of filter decisions
2. **Batch Processing**: Use `process_batch()` methods for higher throughput
3. **Regex Patterns**: Complex regex patterns in content filters can impact performance
4. **Content Filters**: Evaluated for each row, so keep rules simple when possible
5. **Filter Order**: Filters are applied before routing to minimize processing overhead

## Best Practices

1. **Use Wildcards Wisely**: `*` patterns are powerful but can be expensive
2. **Prefer Schema/Table Filters**: More efficient than content-based filters
3. **Cache Filter Configs**: Reuse RdbFilter and RdbRouter instances
4. **Chain Processors**: Use ProcessorChain for clean, maintainable data pipelines
5. **Test Configurations**: Validate filter and routing rules with unit tests
6. **Monitor Performance**: Use metrics to track filtering overhead
7. **Document Rules**: Complex content filters should be well-documented

## Testing

Comprehensive tests are included:

```bash
# Run filter tests
cargo test --package dt-common --lib rdb_filter::tests

# Run router tests
cargo test --package dt-connector --lib rdb_router::tests

# Run processor tests
cargo test --package dt-pipeline --lib filter_processor::tests
cargo test --package dt-pipeline --lib router_processor::tests
cargo test --package dt-pipeline --lib processor_chain::tests

# Run integration tests
cargo test --package dt-pipeline --test filter_router_integration_test
```

## Error Handling

All parsing functions return `anyhow::Result` for comprehensive error handling:

```rust
match RdbFilter::from_config(&config, &db_type) {
    Ok(filter) => { /* use filter */ },
    Err(e) => {
        log::error!("Failed to create filter: {}", e);
        // Handle error appropriately
    }
}
```

## Migration Guide

If upgrading from the basic filtering system:

1. **FilterConfig is backward compatible** - existing configurations still work
2. **New fields are optional** - `content_filters` and `do_cols` default to empty
3. **RouterConfig requires update** - add `content_routes` field (can be empty string)
4. **Test thoroughly** - validate your existing configurations still work as expected

## Future Enhancements

Planned improvements:

- [ ] Dynamic filter reloading without restart
- [ ] Filter performance profiling and metrics
- [ ] Rule composition and reuse
- [ ] Visual filter/router configuration tool
- [ ] Advanced routing strategies (round-robin, weighted)
- [ ] Filter preview/dry-run mode
