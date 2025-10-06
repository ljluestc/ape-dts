# APE-DTS Complete Testing Strategy PRD

Generated: Sun Oct  5 23:16:39 PDT 2025

## Objective
Achieve 100% Unit Testing (UT), Integration Testing, and End-to-End (E2E) coverage for the ape-dts data migration tool.

## Project Overview (from README)

# English | [中文](README_ZH.md)

# Introduction

- ape-dts is a data migration tool enabling any-to-any data transfers.
- It also provides data subscription and data processing.
- It is lightweight, efficient and standalone, requiring no third-party components or extra storage.
- In Rust.

## Key features

- Supports data migration between various databases, both homogeneous and heterogeneous.
- Supports snapshot and cdc tasks with resume from breakpoint.
- Supports checking and revising data.
- Supports filtering and routing at the database, table, and column levels.
- Implements different parallel algorithms for different sources, targets, and task types to improve performance.
- Allows loading user-defined Lua scripts to modify the data.

## Supported task types

|                          | mysql -> mysql | pg -> pg | mongo -> mongo | redis -> redis | mysql -> kafka | pg -> kafka | mysql -> starrocks | mysql -> clickhouse | mysql -> tidb | pg -> starrocks | pg -> clickhouse | mysql -> doris | pg -> doris |
| :----------------------- | :------------- | :------- | :------------- | :------------- | :------------- | :---------- | :----------------- | :------------------ | :------------ | :-------------- | :--------------- | :------------- | :---------- |
| Snapshot                 | &#10004;       | &#10004; | &#10004;       | &#10004;       | &#10004;       | &#10004;    | &#10004;           | &#10004;            | &#10004;      | &#10004;        | &#10004;         | &#10004;       | &#10004;    |
| CDC                      | &#10004;       | &#10004; | &#10004;       | &#10004;       | &#10004;       | &#10004;    | &#10004;           | &#10004;            | &#10004;      | &#10004;        | &#10004;         | &#10004;       | &#10004;    |
| Data check/revise/review | &#10004;       | &#10004; | &#10004;       |                |                |             |                    |                     | &#10004;      |                 |                  |                |             |
| Structure migration      | &#10004;       | &#10004; |                |                |                |             | &#10004;           | &#10004;            | &#10004;      | &#10004;        | &#10004;         | &#10004;       | &#10004;    |

# Advanced

## Crate features

The dt-main crate provides several optional components which can be enabled via `Cargo [features]`:

- `metrics`: Enable Prometheus format task metrics HTTP service interface.
  After enabling this feature, you can customize the metrics service with the following configuration:

  ```
  [metrics]
  # http service host
  http_host=127.0.0.1
  # http service port
  http_port=9090
  # http service worker count
  workers=2
  # prometheus metrics const labels
  labels=your_label1:your_value1,your_label2:your_value2
  ```

- TBD

# Quick starts

## Tutorial

- [prerequisites](./docs/en/tutorial/prerequisites.md)
- [mysql -> mysql](./docs/en/tutorial/mysql_to_mysql.md)
- [pg -> pg](./docs/en/tutorial/pg_to_pg.md)
- [mongo -> mongo](./docs/en/tutorial/mongo_to_mongo.md)
- [redis -> redis](./docs/en/tutorial/redis_to_redis.md)
- [mysql -> starrocks](./docs/en/tutorial/mysql_to_starrocks.md)
- [mysql -> doris](./docs/en/tutorial/mysql_to_doris.md)
- [mysql -> clickhouse](./docs/en/tutorial/mysql_to_clickhouse.md)
- [mysql -> tidb](./docs/en/tutorial/mysql_to_tidb.md)
- [mysql -> kafka -> consumer](./docs/en/tutorial/mysql_to_kafka_consumer.md)
- [pg -> starrocks](./docs/en/tutorial/pg_to_starrocks.md)
- [pg -> doris](./docs/en/tutorial/pg_to_doris.md)
- [pg -> clickhouse](./docs/en/tutorial/pg_to_clickhouse.md)
- [pg -> kafka -> consumer](./docs/en/tutorial/pg_to_kafka_consumer.md)
- [snapshot + cdc without data loss](./docs/en/tutorial/snapshot_and_cdc_without_data_loss.md)
- [modify data by lua](./docs/en/tutorial/etl_by_lua.md)

## Run tests

Refer to [test docs](./dt-tests/README.md) for details.

# More docs

- Configurations
  - [config details](./docs/en/config.md)
- Structure tasks
  - [migration](./docs/en/structure/migration.md)
  - [check](./docs/en/structure/check.md)
  - [check by Liquibase](./docs/en/structure/check_by_liquibase.md)
- Snapshot tasks
  - [data migration](./docs/en/snapshot/migration.md)
  - [data check](./docs/en/snapshot/check.md)
  - [data revise](./docs/en/snapshot/revise.md)
  - [data review](./docs/en/snapshot/review.md)
  - [resume at breakpoint](./docs/en/snapshot/resume.md)
  - [multiple tables in parallel](./docs/en/snapshot/tb_in_parallel.md)
- CDC tasks
  - [data sync](./docs/en/cdc/sync.md)
  - [heartbeat to source database](./docs/en/cdc/heartbeat.md)
  - [two-way data sync](./docs/en/cdc/two_way.md)
  - [generate sqls from CDC](./docs/en/cdc/to_sql.md)
  - [resume at breakpoint](./docs/en/cdc/resume.md)
- Custom consumers
  - [mysql/pg -> kafka -> consumer](./docs/en/consumer/kafka_consumer.md)
- Data processing
  - [modify data by lua](./docs/en/etl/lua.md)
- Monitor
  - [monitor info](./docs/en/monitor/monitor.md)
  - [position info](./docs/en/monitor/position.md)
- Task templates
  - [mysql -> mysql](./docs/templates/mysql_to_mysql.md)
  - [pg -> pg](./docs/templates/pg_to_pg.md)
  - [mongo -> mongo](./docs/templates/mongo_to_mongo.md)
  - [redis -> redis](./docs/templates/redis_to_redis.md)
  - [mysql/pg -> kafka](./docs/templates/rdb_to_kafka.md)
  - [mysql -> starrocks](./docs/templates/mysql_to_starrocks.md)
  - [mysql -> doris](./docs/templates/mysql_to_doris.md)
  - [mysql -> clickhouse](./docs/templates/mysql_to_clickhouse.md)
  - [pg -> starrocks](./docs/templates/pg_to_starrocks.md)
  - [pg -> doris](./docs/templates/pg_to_doris.md)
  - [pg -> clickhouse](./docs/templates/pg_to_clickhouse.md)

# Benchmark

- MySQL -> MySQL, Snapshot

| Method   | Node Specs | RPS(rows per second) | Source MySQL Load (CPU/Memory) | Target MySQL Load (CPU/Memory) |
| :------- | :--------- | :------------------- | :----------------------------- | :----------------------------- |
| ape_dts  | 1c2g       | 71428                | 8.2% / 5.2%                    | 211% / 5.1%                    |
| ape_dts  | 2c4g       | 99403                | 14.0% / 5.2%                   | 359% / 5.1%                    |
| ape_dts  | 4c8g       | 126582               | 13.8% / 5.2%                   | 552% / 5.1%                    |
| debezium | 4c8g       | 4051                 | 21.5% / 5.2%                   | 51.2% / 5.1%                   |

- MySQL -> MySQL, CDC

| Method   | Node Specs | RPS(rows per second) | Source MySQL Load (CPU/Memory) | Target MySQL Load (CPU/Memory) |
| :------- | :--------- | :------------------- | :----------------------------- | :----------------------------- |
| ape_dts  | 1c2g       | 15002                | 18.8% / 5.2%                   | 467% / 6.5%                    |
| ape_dts  | 2c4g       | 24692                | 18.1% / 5.2%                   | 687% / 6.5%                    |
| ape_dts  | 4c8g       | 26287                | 18.2% / 5.2%                   | 685% / 6.5%                    |
| debezium | 4c8g       | 2951                 | 20.4% / 5.2%                   | 98% / 6.5%                     |

- Image size

| ape_dts:2.0.25-alpha.1 | debezium/connect:2.7 |
| :--------------------- | :------------------- |
| 71.4 MB                | 1.38 GB              |

- more benchmark [details](./docs/en/benchmark.md)

# Contributing

## Structure

![Structure](docs/img/structure.png)

## Modules

- dt-main: program entry
- dt-precheck: pre-check, to minimize interruptions during subsequent data operations by identifying issues early for fast failure
- dt-connector: extractors + sinkers for databases
- dt-pipeline: pipeline to connect extractors and sinkers
- dt-parallelizer: parallel algorithms
- dt-task: create extractors + sinkers + pipelines + parallelizers according to configurations
- dt-common: common utils, basic data structures, metadata management
- dt-tests: integration tests

- related sub module: [mysql binlog connector in rust](https://github.com/apecloud/mysql-binlog-connector-rust)

## Build

- Minimum supported Rust version (MSRV)
  The current minimum supported Rust version (MSRV) is 1.85.0.
- cargo build
- [build images](./docs/en/build_images.md)

## Checklist

- run `cargo clippy --all-targets --all-features --workspace` fix all clippy issues.

# Contact us

[Slack Community](https://join.slack.com/t/kubeblocks/shared_invite/zt-22cx2f84x-BPZvnLRqBOGdZ_XSjELh4Q)

## Existing Documentation Reference

### Document: docs/en/cdc/heartbeat.md

# Enable heartbeat to source database

CDC tasks calculate delays by positions. For example, a MySQL CDC task uses the synced source binlog offset as the position. The position should be consistent with the source database if the CDC task catches up, and the timestamp of the position (if any) should follow the current time.

But if the source database has not been updated for a long time, or there are updates but the updated tables are not subscribed by the CDC task, then the position won't change, which will be considered as a delay. Therefore, we can create a heartbeat table in the source database and update the table periodically by CDC tasks to push the task position forward.

# Configurations

- For MySQL/PG/Mongo, refer to:
    - dt-tests/tests/mysql_to_mysql/cdc/heartbeat_test
    - dt-tests/tests/pg_to_pg/cdc/heartbeat_test
    - dt-tests/tests/mongo_to_mongo/cdc/heartbeat_test

```
[extractor]
heartbeat_interval_secs=10
heartbeat_tb=test_db_1.ape_dts_heartbeat

[filter]
ignore_tbs=test_db_1.ape_dts_heartbeat
```

- For Redis，refer to: dt-tests/tests/redis_to_redis/cdc/heartbeat_test
```
[extractor]
heartbeat_interval_secs=10
heartbeat_key=5.ape_dts_heartbeat_key
```

# Heartbeat table

- MySQL
```
CREATE TABLE IF NOT EXISTS `{}`.`{}`(
    server_id INT UNSIGNED,
    update_timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    received_binlog_filename VARCHAR(255),
    received_next_event_position INT UNSIGNED,
    received_timestamp VARCHAR(255),
    flushed_binlog_filename VARCHAR(255),
    flushed_next_event_position INT UNSIGNED,
    flushed_timestamp VARCHAR(255),
    PRIMARY KEY(server_id)
)
```

- PG
```
CREATE TABLE IF NOT EXISTS "{}"."{}"(
    slot_name character varying(64) not null,
    update_timestamp timestamp without time zone default (now() at time zone 'utc'),
    received_lsn character varying(64),
    received_timestamp character varying(64),
    flushed_lsn character varying(64),
    flushed_timestamp character varying(64),
    primary key(slot_name)
)
```

Note:
- The names of databases and tables should be the same with those of heartbeat_tb in task_config.ini.
- No need to create heartbeat tables for Mongo and Redis.
- Keep heartbeat_tb empty if not needed.
- If heartbeat_tb is configured but the table is NOT created, CDC task will try to create the table automatically. So, the extractor account needs to have corresponding permissions.
---

### Document: docs/en/cdc/resume.md

# Resume at breakpoint

Task progress will be recorded periodically in position.log.

If task interrupts, you need to restart it manually. By default, it will start from [extractor] configs in task_config.ini.

To avoid syncing duplicate data, the task can resume at the breakpoint in finished.log.

Since resuming depends on position.log, if you have multiple tasks, **you must set up separate log directories for each task**.

## Supported
- MySQL as source
- Postgres as source
- Mongo as source

# Position Info
[position Info](../monitor/position.md)

## MySQL position.log
```
2024-10-10 08:01:09.308022 | checkpoint_position | {"type":"MysqlCdc","server_id":"","binlog_filename":"mysql-bin.000036","next_event_position":773,"gtid_set":"","timestamp":"2024-10-10 08:00:58.000"}
```

## Postgres position.log
```
2024-10-10 09:09:52.260052 | checkpoint_position | {"type":"PgCdc","lsn":"0/406E2C30","timestamp":"2024-10-10 08:12:31.421"}
```

## Mongo position.log 
### op_log
```
2024-10-10 09:17:14.825459 | current_position | {"type":"MongoCdc","resume_token":"","operation_time":1728551829,"timestamp":"2024-10-10 09:17:09.000"}
```

### change_stream
```
2024-10-10 08:46:34.218284 | current_position | {"type":"MongoCdc","resume_token":"{\"_data\":\"8267079350000000012B022C0100296E5A1004B4A9FD2BFD9C44609366CD4CD6A3D98E46645F696400646707935067D762990668C8CE0004\"}","operation_time":1728549712,"timestamp":"2024-10-10 08:41:52.000"}
```

# Configurations

CDC resume configuration is similar to [snapshot task](../snapshot/resume.md), please read first to understand its principles.

Differences:
- MySQL/Postgres position info will load from checkpoint_position in position.log.
- Mongo position info will load from current_position in position.log.

# Example 1

- task_config.ini
```
[extractor]
db_type=mysql
extract_type=cdc
binlog_position=73351
binlog_filename=mysql-bin.000004

[resumer]
resume_from_log=true
```

- position.log generated before the task was interrupted:
```
2024-10-18 05:21:45.207788 | checkpoint_position | {"type":"MysqlCdc","server_id":"","binlog_filename":"mysql-bin.000004","next_event_position":73685,"gtid_set":"","timestamp":"2024-10-18 05:21:44.000"}
```

After task restarts, default.log:

```
2024-10-18 07:34:29.702024 - INFO - [1256892] - resume from: {"type":"MysqlCdc","server_id":"","binlog_filename":"mysql-bin.000004","next_event_position":73685,"gtid_set":"","timestamp":"2024-10-18 05:21:44.000"}
2024-10-18 07:34:29.702621 - INFO - [1256892] - MysqlCdcExtractor starts, binlog_filename: mysql-bin.000004, binlog_position: 73685, gtid_enabled: false, gtid_set: , heartbeat_interval_secs: 1, heartbeat_tb: heartbeat_db.ape_dts_heartbeat
```

# Example 2
- task_config.ini
```
[extractor]
db_type=mysql
extract_type=cdc
binlog_position=73351
binlog_filename=mysql-bin.000004

[resumer]
resume_config_file=./resume.config
```

- ./resume.config (filled in by user)
```
2024-10-18 05:21:45.207788 | checkpoint_position | {"type":"MysqlCdc","server_id":"","binlog_filename":"mysql-bin.000004","next_event_position":73685,"gtid_set":"","timestamp":"2024-10-18 05:21:44.000"}
```

After task restarts, default.log:

```
2024-10-18 07:40:02.283542 - INFO - [1267442] - resume from: {"type":"MysqlCdc","server_id":"","binlog_filename":"mysql-bin.000004","next_event_position":73685,"gtid_set":"","timestamp":"2024-10-18 05:21:44.000"}
2024-10-18 07:40:02.284100 - INFO - [1267442] - MysqlCdcExtractor starts, binlog_filename: mysql-bin.000004, binlog_position: 73685, gtid_enabled: false, gtid_set: , heartbeat_interval_secs: 1, heartbeat_tb: heartbeat_db.ape_dts_heartbeat
```
---

### Document: docs/en/cdc/sync.md

# Sync CDC data

Subscribe to data changes in the source database and sync them to the target.

Prerequisites
- MySQL: Enables binlog in the source database;
- PG: Sets `wal_level = logical` in the source database;
- Mongo: The source instance must be ReplicaSet;
- For more information, refer to [init test env](../../../dt-tests/README.md).

# Example: MySQL -> MySQL

Refer to [task templates](../../templates/mysql_to_mysql.md) and [tutorial](../tutorial/mysql_to_mysql.md)

# Parallelizer

- MySQL/PG: parallel_type=rdb_merge
- Mongo: parallel_type=mongo
- Redis: parallel_type=redis

# Other configurations

- For [filter] and [router], refer to [config details](../config.md).
- Refer to task_config.ini in tests:
    - dt-tests/tests/mysql_to_mysql/cdc
    - dt-tests/tests/pg_to_pg/cdc
    - dt-tests/tests/mongo_to_mongo/cdc
    - dt-tests/tests/redis_to_redis/cdc

- Modify performance parameters if needed:
```
[pipeline]
buffer_size=16000
checkpoint_interval_secs=10

[sinker]
batch_size=200

[parallelizer]
parallel_size=8
```
---

### Document: docs/en/cdc/to_sql.md

# Generate sqls from CDC data

Subscribe to data changes in the source database and generate sqls / reverse sqls, stored in sql.log.

Supported databases:
- MySQL
- PG

Supported data changes:
- DML

# Examples

Refer to [task templates](../../templates/mysql_to_mysql.md)

## Example 1: generate sqls
```
[extractor]
db_type=mysql
extract_type=cdc
binlog_position=0
binlog_filename=
server_id=2000
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[filter]
do_tbs=test_db.*
do_events=insert,update,delete

[sinker]
db_type=mysql
sink_type=sql
```

execute sqls in source:
```
use test_db;
insert into test_tb values(1, 1);
update test_tb set value=2 where id=1;
delete from test_tb where id=1;
```

generated sql.log:
```
INSERT INTO `test_db`.`test_tb`(`id`,`value`) VALUES(1,1);
UPDATE `test_db`.`test_tb` SET `id`=1,`value`=2 WHERE `id` = 1;
DELETE FROM `test_db`.`test_tb` WHERE `id` = 1;
```

## Example 2: generate reverse sqls
Add configs based on example 1:

```
[sinker]
reverse=true
```

execute sqls in source:
```
use test_db;
insert into test_tb values(1, 1);
update test_tb set value=2 where id=1;
delete from test_tb where id=1;
```

generated sql.log:
```
DELETE FROM `test_db`.`test_tb` WHERE `id` = 1;
UPDATE `test_db`.`test_tb` SET `id`=1,`value`=1 WHERE `id` = 1;
INSERT INTO `test_db`.`test_tb`(`id`,`value`) VALUES(1,2);
```

## Set start and end time
If you need data changes within a certain period of time, add configs:

```
[extractor]
start_time_utc=2024-10-09 02:00:00
end_time_utc=2024-10-09 03:00:00
```

# Data rollback
If some sqls were executed incorrectly in the source and you want to roll back the data, you may:
- generate reverse sqls and execute them from the last to the first.


---

### Document: docs/en/cdc/two_way.md

# Two-way CDC data synchronization

In a distributed system, the source and target databases may not necessarily have a simple master-slave relationship. Sometimes they can independently accept data changes, but data sync is still needed to ensure that they both have complete data.

To enable two-way data sync, we need to configure CDC tasks for both "source -> target" and "target -> source" directions.

# Cyclic replication

The main challenge for two-way data sync is to avoid data cyclic replication. Considering the following scenario:

- A record "a" is inserted into the source MySQL database, and the CDC task "source -> target" parses this change from the source's binlog and syncs it into the target.
- A binlog record for inserting "a" is generated in the target MySQL.
- CDC task "target -> source" pulls the binlog entry for inserting record "a", and syncs it to source.

# Topology

In fact, two-way or even net data sync can be considered as a topology:

- Databases that need to be synced with each other form a logical cluster, and each database is a node.
- One or more data sync tasks are used to connect these databases, and each task is a side. Data is synced between these nodes.
- Topology = database nodes + tasks.

## Examples

<div align=center>
<img src="../../img/topo_two_way.png" width="55%" />
<br/>
Two-Way Topology
</div>

***

<div align=center>
<img src="../../img/topo_net.png"/>
<br/>
Net Topology
</div>

***

<div align=center>
<img src="../../img/topo_star.png"/>
<br/>
Star Topology
</div>

***

# Data marking

We use data marker to avoid cyclic replication.

## Principles

Assume that in a MySQL two-way topology, there are two tasks:

- task1 (source: node1, target: node2).
- task2 (source: node2, target: node1).

When task1 writes data to node2, it generates a marker that identifies the original source of the data. This marker is recorded in the binlog of node2.

When task2 parses the binlog from node2, it finds out that some binlogs are generated by task1, and the original source of the data is node1, so it gives up these data to avoid cyclic data sync.

## Marker table

In order to mark CDC data, the target database needs an additional marker table. The marker info is actually written to binlog(mysql) /wal(pg) / aof(redis) by updating the marker table.

If data marker is configured with no table created, CDC tasks will try to create it. So, the sinker account needs to have corresponding permissions.

### MySQL
```
CREATE TABLE IF NOT EXISTS `{}`.`{}` (
    data_origin_node varchar(255) NOT NULL,
    src_node varchar(255) NOT NULL,
    dst_node varchar(255) NOT NULL,
    n bigint DEFAULT NULL,
    PRIMARY KEY (data_origin_node, src_node, dst_node)
)
```

### PG
```
CREATE TABLE IF NOT EXISTS "{}"."{}" (
    data_origin_node varchar(255) NOT NULL,
    src_node varchar(255) NOT NULL,
    dst_node varchar(255) NOT NULL,
    n bigint DEFAULT NULL,
    PRIMARY KEY (data_origin_node, src_node, dst_node)
)
```

### Redis

The Redis marker info is written to the target aof by updating the Redis key. No need to pre-create anything.


# Configurations

In addition to the regular configurations, you also need to add the [data_marker] configuration.

```
[data_marker]
topo_name=topo1
topo_nodes=node1,node2
src_node=node1
dst_node=node2
do_nodes=node1
ignore_nodes=node2
marker=ape_trans_mysql.topo1
```

- topo_name: the name of topology, defined by the user. It should be consistent across all tasks within the topology.
- topo_nodes: node names in topology, defined by the user. It should be consistent across all tasks within the topology. This field is reserved and has not been used yet.
- src_node: source node of the current task.
- dst_node: target node of the current task.
- do_nodes: If the data is originated from these nodes, the task will sync it to the target.
- ignore_nodes: If the data is originated from these nodes, the task will ignore it.
- marker: data marker table, defined by the user. It should be consistent across all tasks within the topology.



# Example: MySQL -> MySQL

Here is an example for two-way data sync.

## node1 -> node2
```
[data_marker]
topo_name=topo1
topo_nodes=node1,node2
src_node=node1
dst_node=node2
do_nodes=node1
ignore_nodes=node2
marker=ape_trans_mysql.topo1
```

## node2 -> node1
```
[data_marker]
topo_name=topo1
topo_nodes=node1,node2
src_node=node2
dst_node=node1
do_nodes=node2
ignore_nodes=node1
marker=ape_trans_mysql.topo1
```

# Other configurations

Refer to task_config.ini in cycle-related tests:
- dt-tests/tests/mysql_to_mysql/cdc
- dt-tests/tests/pg_to_pg/cdc
- dt-tests/tests/redis_to_redis/cdc
---

### Document: docs/en/consumer/http_consumer.md

# Start ape_dts as an HTTP server to provide data to consumers

Refer to [tutorial](/docs/en/tutorial/mysql_to_http_server_consumer.md)

ape_dts starts as an HTTP server, pulling CDC data from MySQL/Postgres and cache it in memory. 

Consumers can pull and consume data from ape_dts via API, the data format is Avro, same to [MySQL -> Kafka](/docs/en/consumer/kafka_consumer.md)

Snapshot task is NOT supported since it is more convenient to query data through SQL and consume.

# Api

## info
Get the current information of the server.

curl "http://127.0.0.1:10231/info"

### Response
```
{"acked_batch_id":0,"sent_batch_id":0}
```

- batch_id: Generated by ape_dts, increments by 1 with each data pull by the consumer, starting from 0, and reset when ape_dts restarts.
- sent_batch_id: The maximum batch_id that has been sent to the client.
- acked_batch_id: The maximum batch_id that has been acknowledged by the client, acknowledged data will be removed from ape_dts's cache.

## fetch_new
Fetch new data from the server.

curl "http://127.0.0.1:10231/fetch_new?batch_size=2&ack_batch_id=1"

### Parameters
- batch_size: The maximum records count to pull, if ape_dts's cache is insufficient, all available data will be returned.
- ack_batch_id: Optional.
    - If set, data with batch_id <= ack_batch_id will be removed from ape_dts's cache.
    - ack_batch_id must be >= acked_batch_id returned by info.
    - ack_batch_id must be <= sent_batch_id returned by info.

### Response

```
{"data":[[14,116,101,115,116,95,100,98,8,116,98,95,49,12,105,110,115,101,114,116,2,4,4,105,100,6,105,110,116,8,76,111,110,103,10,118,97,108,117,101,6,105,110,116,8,76,111,110,103,0,0,2,4,4,105,100,4,2,10,118,97,108,117,101,4,2,0,0],[14,116,101,115,116,95,100,98,8,116,98,95,49,12,105,110,115,101,114,116,2,4,4,105,100,6,105,110,116,8,76,111,110,103,10,118,97,108,117,101,6,105,110,116,8,76,111,110,103,0,0,2,4,4,105,100,4,4,10,118,97,108,117,101,4,4,0,0]],"batch_id":1}
```

- data: Multiple data entries encoded in Avro format. Refer to the [Parse and Consume] later in this article.
- batch_id: The ID generated by ape_dts for this pull.

## fetch_old

Fetch old data repeatedly from the server.

curl "http://127.0.0.1:10232/fetch_old?old_batch_id=1"

### Parameters

- old_batch_id: The batch_id of the old data to be fetched.
    - old_batch_id must be <= sent_batch_id returned by info.
    - old_batch_id must be > acked_batch_id returned by info.

### Response

```
{"data":[[14,116,101,115,116,95,100,98,8,116,98,95,49,12,105,110,115,101,114,116,2,4,4,105,100,6,105,110,116,8,76,111,110,103,10,118,97,108,117,101,6,105,110,116,8,76,111,110,103,0,0,2,4,4,105,100,4,2,10,118,97,108,117,101,4,2,0,0],[14,116,101,115,116,95,100,98,8,116,98,95,49,12,105,110,115,101,114,116,2,4,4,105,100,6,105,110,116,8,76,111,110,103,10,118,97,108,117,101,6,105,110,116,8,76,111,110,103,0,0,2,4,4,105,100,4,4,10,118,97,108,117,101,4,4,0,0]],"batch_id":1}
```

- Same with fetch_new.

## ack

Send acknowledgement to ape_dts.

curl -X POST "http://127.0.0.1:10232/ack" -H "Content-Type: application/json" -d '{"ack_batch_id": 6}'

### Parameters

- ack_batch_id: Same as the ack_batch_id parameter in fetch_new.

### Response
```
{"acked_batch_id":1}
```

- acked_batch_id: Same as the acked_batch_id returned by info.


# Parse and Consume

[python / golang consumer demo](https://github.com/apecloud/ape_dts_consumer_demo)
---

### Document: docs/en/consumer/kafka_consumer.md

# Send data to Kafka for consumers

The Snapshot/CDC data will be sent to Kafka in Avro.

# Send data to Kafka

Refer to MySQL -> Kafka [tutorial](/docs/en/tutorial/mysql_to_kafka_consumer.md), [templates](/docs/templates/rdb_to_kafka.md) and Postgres -> Kafka [tutorial](/docs/en/tutorial/pg_to_kafka_consumer.md), [templates](/docs/templates/rdb_to_kafka.md)

# Consumer

[python / golang consumer demo](https://github.com/apecloud/ape_dts_consumer_demo)
---

### Document: docs/en/etl/lua.md

# Modify data by Lua

During data sync, a user may need to modify the data manually.

In ape-dts tasks, the user can provide a Lua script to process each row data, such as:

- Add columns
- Drop columns
- Modify column values
- Modify schema name / table name / column name
- Modify row type, eg: change update data to insert data
- Filter rows

# Config
- in task_config.ini
```
[processor]
lua_code_file=./dt-tests/tests/mysql_to_mysql_lua/cdc/basic_test/src_to_dst/lua_code.lua
```

# How it works
The task passes each row data pulled by the extractor into Lua as global variables, and then executes user code to process the data.

Processed data will continue subsequent processes of the task.


- Global variables passed into Lua:

| Variable | Data Type | Description |
| :-------- | :-------- | :-------- |
| schema | string | database name(mysql) / schema name(postgres) |
| tb | string | table name |
| row_type | string | row data type: insert / update / delete |
| before | table | exists in update / delete row data, key for column name，value for column value |
| after | table | exists in  update / insert row data, key for column name，value for column value |

# Examples
## Add columns
```
if (schema == "lua_test" and tb == "add_column_test" and row_type == "insert")
then
    after["new_column"] = 1000
end
```

## Drop columns
```
if (schema == "lua_test" and tb == "drop_column_test" and row_type == "insert")
then
    after["column_1"] = nil
end
```

## Modify column values
```
if (schema == "lua_test" and tb == "change_column_value_test" and row_type == "insert")
then
    after["column_1"] = "new_value"
end
```

## Change column name
```
if (schema == "lua_test" and tb == "change_column_name_test" and row_type == "insert")
then
    after["f_1_1"] = after.f_1
    after["f_1"] = nil
end
```

## Change table name
```
if (schema == "lua_test" and tb == "change_table_name_test")
then
    tb = "change_table_name_test_dst"
end
```

## Filter rows
- set row_type to "", the row will be filtered
```
if schema == "lua_test" and tb == "filter_row_test" then
    if (after.create_time ~= nil and after.create_time < '2024-12-01 00:00:00') or
       (before.create_time ~= nil and before.create_time < '2024-12-01 00:00:00') then
        row_type = ""
    end
end
```

## More references
- test cases: dt-tests/tests/mysql_to_mysql_lua, dt-tests/tests/pg_to_pg_lua

# Supported scenarios
- Only for snapshot / cdc tasks whose source is mysql / postgres.
- For cdc tasks, only supports processing on dml data.
- For binary columns, eg: mysql tinyblob, mediumblob, longblob, blob, varbinary, binary, currently:
    - Dropping these columns is supported.
    - Modifying them is NOT supported.

---

### Document: docs/en/monitor/monitor.md

# Monitoring info
Counters are used to record the task status, they will be periodically logged in monitor.log (configuration: [pipeline] checkpoint_interval_secs).

# Time window counters
This type of counter is an array of sub-counters. During task execution, whenever there is a state change (e.g., successfully writing a batch entries to target), a new sub-counter is generated to record the increment info (e.g., number of entries written to target).

- The counter has a time window (configuration: [pipeline] counter_time_window_secs), expired sub-counters will be discarded.
- The counter is used for real-time monitoring, such as the number of synchronized entries in time window.
- The counter has aggregation algorithms, such as the average count of synchronized entries per second.

## Aggregation algorithms

| Aggregation | Description | Example |
| :-------- | :-------- | :-------- | 
| sum | sum of sub-counters | count of synchronized entries in last 10 seconds |
| avg | sum of sub-counters / number of sub-counters | average time cost for each write to target in last 10 seconds |
| avg_by_sec | sum of all sub-counters / time window | average number of entries written to target per second in last 10 seconds |
| max | the sub-counter with the maximum value | maximum number of entries written to target in a single batch in last 10 seconds |
| max_by_sec | sums the sub-counters for each second, and finds the second with the maximum sum | Maximum number of entries written to target in a single second |

# No window counter

A simple counter to record accumulated data, such as the number of migrated MySQL records.

## Aggregation algorithms

| Aggregation | Description | Example |
| :-------- | :-------- | :-------- |
| latest | Current value | Number of synchronized data entries by the task |


# Counter details

## Time window configuration

```
[pipeline]
counter_time_window_secs=60
```

## extractor
### monitor.log
```
2024-02-29 01:25:09.554271 | extractor | record_count | avg_by_sec=13 | sum=13 | max_by_sec=13
2024-02-29 01:25:09.554311 | extractor | data_bytes | avg_by_sec=586 | sum=586 | max_by_sec=586
```

### counters
| Counter | Counter Type | Description |
| :-------- | :-------- | :-------- |
| record_count | time window | Number of data entries pulled |
| data_bytes | time window | Data bytes pulled |

<br/>

- record_count

| Aggregation | Description |
| :-------- | :-------- |
| avg_by_sec | Average number of entries pulled per second in time window |
| sum | Number of entries pulled in time window |
| max_by_sec | Maximum number of entries pulled per second in time window |

<br/>

- data_bytes

| Aggregation | Description |
| :-------- | :-------- |
| avg_by_sec | Average data bytes pulled per second in time window |
| sum | Data bytes pulled in time window |
| max_by_sec | Maximum data bytes pulled per second in window |

## sinker

### monitor.log

```
2024-02-29 01:25:09.554461 | sinker | rt_per_query | avg=3369 | sum=23585 | max=6408
2024-02-29 01:25:09.554503 | sinker | record_count | avg_by_sec=13 | sum=13 | max_by_sec=13
2024-02-29 01:25:09.554544 | sinker | data_bytes | avg_by_sec=586 | sum=586 | max_by_sec=586
2024-02-29 01:25:09.554582 | sinker | records_per_query | avg=1 | sum=13 | max=2
```

### counter Description

| counter | Counter Type | Description |
| :-------- | :-------- | :-------- |
| rt_per_query | time window | Time taken for a single write, in microseconds |
| records_per_query | time window | Number of entries per single write |
| record_count | time window | Number of entries written to target |
| data_bytes | time window |Data bytes written to target |

<br/>

- rt_per_query

| Aggregation | Description |
| :-------- | :-------- |
| avg | Average time taken for a single write in window |
| sum | Total time taken for writes to target in window |
| max | Maximum time taken for a single write in window |

<br/>

- record_count

| Aggregation | Description |
| :-------- | :-------- |
| avg_by_sec | Average number of entries written per second in window |
| sum | Total number of entries written in window |
| max_by_sec | Maximum number of entries written per second in window |

<br/>

- data_bytes

| Aggregation | Description |
| :-------- | :-------- |
| avg_by_sec | Average bytes written per second in window |
| sum | Total bytes written in window |
| max_by_sec |Maximum bytes written per second in window |

<br/>

- records_per_query

| Aggregation | Description |
| :-------- | :-------- |
| avg | Average number of entries per query in window |
| sum | Total number of entries written in window |
| max | Maximum number of entries written per query in window |


## pipeline
### monitor.log
```
2024-02-29 01:25:09.554348 | pipeline | record_size | avg=45
2024-02-29 01:25:09.554387 | pipeline | buffer_size | avg=3 | sum=13 | max=4
2024-02-29 01:25:09.554423 | pipeline | sinked_count | latest=13
```

### counter Description

| Counter | Counter Type | Description |
| :-------- | :-------- | :-------- |
| record_size | time window | Size of a single entry, in bytes |
| buffer_size | time window | Number of entries cached in pipeline |
| sinked_count | no window | Total Number of entries handled by task |

<br/>

- record_size

| Aggregation | Description |
| :-------- | :-------- |
| avg | Average size of each entry in window |

<br/>

- buffer_size

| Aggregation | Description |
| :-------- | :-------- |
| avg | Average number of cached entries in window |
| sum | Total number of cached entries in window |
| max | Maximum number of cached entries in window |

<br/>

- sinked_count

| Aggregation | Description |
| :-------- | :-------- |
| latest | Number of entries handled by task |
---

### Document: docs/en/monitor/position.md

# Task progress info

Task progress will be recorded periodically in position.log(configuration: [pipeline] checkpoint_interval_secs).

# CDC

For CDC tasks, we only guarantee eventual consistency between target and source, binlog/wal for a big transaction in source may be synced to target by multiple parts. Thus, we will record both current_position and checkpoint_position in position.log.

- current_position: position of synced data, may be in the middle of a large transaction binlog/wal.
- checkpoint_position: position of the last synced transaction binlog/wal.

If task interrupts, use checkpoint_position as the starting point for new task, refer to [CDC task resume](../cdc/resume.md), using current_position may cause errors when parsing binlog/wal.

## MySQL

Depends on gtid enabled or not, refer to [tutorial](./tutorial/mysql_to_mysql.md):

- Use binlog_filename + next_event_position as position if gtid disabled.

```
2024-10-18 05:21:45.207788 | checkpoint_position | {"type":"MysqlCdc","server_id":"","binlog_filename":"mysql-bin.000004","next_event_position":44315,"gtid_set":"","timestamp":"2024-10-18 05:21:44.000"}
```

- Use gtid_set as position if gtid enabled.
```
2024-10-18 05:22:41.201208 | checkpoint_position | {"type":"MysqlCdc","server_id":"","binlog_filename":"mysql-bin.000004","next_event_position":50865,"gtid_set":"9663a096-8adc-11ef-b617-0242ac110002:1-3112","timestamp":"2024-10-18 05:22:41.000"}
```

## Postgres

Use lsn as position.

```
2024-10-18 05:22:22.419787 | checkpoint_position | {"type":"PgCdc","lsn":"0/5D65CB0","timestamp":"2024-10-18 05:22:21.756"}
```

## Mongo

Only current_position for Mongo, depends on source types:

- Use operation_time as position ([extractor] source=op_log).

```
2024-10-18 05:19:25.877182 | current_position | {"type":"MongoCdc","resume_token":"","operation_time":1729228763,"timestamp":"2024-10-18 05:19:23.000"}
```

- Use resume_token as position ([extractor] source=change_stream).

```
2024-10-18 05:20:33.977700 | current_position | {"type":"MongoCdc","resume_token":"{\"_data\":\"826711F020000000042B022C0100296E5A10040E19213A975845EBAD0B8384EAE1DA1C46645F696400646711F01A88DC948E321DEC2A0004\"}","operation_time":1729228832,"timestamp":"2024-10-18 05:20:32.000"}
```


## Redis

Use repl_offset as position.

```
2024-10-18 05:23:41.019195 | checkpoint_position | {"type":"Redis","repl_id":"1cd12b27acff56526106e343b9f4ff623b5e4c14","repl_port":10008,"repl_offset":2056,"now_db_id":0,"timestamp":""}
```

# Snapshot

If the snapshot task contains multiple databases/tables, tables will be sorted **first by database name and then table name**, and they will be migrated to the target **one by one**.

If a table has a **single column** **primary key/unique key**, extractor will use it as sorting column and pull data in batches(configuration: [extractor] batch_size) from small to large, otherwise the table will be pulled in a stream.

## default.log

Once a table migrating starts/ends, in default.log:

```
2024-02-28 10:07:35.531681 - INFO - [14778588] - start extracting data from `test_db_1`.`one_pk_no_uk` by slices
2024-02-28 10:07:35.735439 - INFO - [14778588] - end extracting data from `test_db_1`.`one_pk_no_uk`, all count: 9
```

## finished.log

Once a table migrating ends, in finished.log:

```
2024-10-10 04:04:07.803422 | {"type":"RdbSnapshotFinished","db_type":"mysql","schema":"test_db","tb":"a"}
```

## position.log

The progress of migrating tables will be logged in position.log.
Use the sorting column's value of the **last migrated record** as position.

### MySQL

```
2024-10-10 04:04:08.152044 | current_position | {"type":"RdbSnapshot","db_type":"mysql","schema":"test_db","tb":"b","order_col":"id","value":"6"}
```

### Postgres

```
2024-10-10 04:04:09.223040 | current_position | {"type":"RdbSnapshot","db_type":"pg","schema":"test_db","tb":"b","order_col":"id","value":"6"}
```

### Mongo

```
2024-10-18 04:10:35.792078 | current_position | {"type":"RdbSnapshot","db_type":"mongo","schema":"test_db_2","tb":"tb_1","order_col":"_id","value":"6711dfb9643426296f0cb93d"}
```

### Redis
```
2024-10-18 05:24:47.932794 | current_position | {"type":"Redis","repl_id":"1cd12b27acff56526106e343b9f4ff623b5e4c14","repl_port":10008,"repl_offset":5103,"now_db_id":0,"timestamp":""}
```

---

### Document: docs/en/snapshot/check.md

# Check data

After data migration, you may want to compare the source data and the target data. If there are too many records, try sampling check. Before you start, please ensure that the tables to be verified have primary/unique keys.

MySQL/PG/Mongo are currently supported for data check.

# Example: MySQL -> MySQL

Refer to [task templates](../../templates/mysql_to_mysql.md) and [tutorial](../tutorial/mysql_to_mysql.md)

## Sampling check

Based on full check configuration, add `sample_interval` for sampling check. The following code means that every 3 records will be sampled once.

```
[extractor]
sample_interval=3
```

## Note

While this configuration is similar to that of snapshot migration, the only differences are:

```
[sinker]
sink_type=check

[parallelizer]
parallel_type=rdb_check
```

# Results

The results are written to logs in JSON format, including diff.log and miss.log. The logs are stored in the log/check subdirectory.

## diff.log

The diff log includes the database (schema), table (tb), primary key/unique key (id_col_values), and the source and target values of the differing columns (diff_col_values).

```
{"log_type":"Diff","schema":"test_db_1","tb":"one_pk_multi_uk","id_col_values":{"f_0":"5"},"diff_col_values":{"f_1":{"src":"5","dst":"5000"}}}
{"log_type":"Diff","schema":"test_db_1","tb":"one_pk_no_uk","id_col_values":{"f_0":"4"},"diff_col_values":{"f_1":{"src":"2","dst":"1"}}}
{"log_type":"Diff","schema":"test_db_1","tb":"one_pk_no_uk","id_col_values":{"f_0":"6"},"diff_col_values":{"f_1":{"src":null,"dst":"1"}}}
```

## miss.log

The miss log includes the database (schema), table (tb), and primary key/unique key (id_col_values), with empty diff_col_values.

```
{"log_type":"Miss","schema":"test_db_1","tb":"no_pk_one_uk","id_col_values":{"f_1":"8","f_2":"1"},"diff_col_values":{}}
{"log_type":"Miss","schema":"test_db_1","tb":"no_pk_one_uk","id_col_values":{"f_1":null,"f_2":null},"diff_col_values":{}}
{"log_type":"Miss","schema":"test_db_1","tb":"one_pk_multi_uk","id_col_values":{"f_0":"7"},"diff_col_values":{}}
```

# Other configurations

- For [filter] and [router], refer to [config details](../config.md).
- Refer to task_config.ini in tests:
    - dt-tests/tests/mysql_to_mysql/check
    - dt-tests/tests/pg_to_pg/check
    - dt-tests/tests/mongo_to_mongo/check
---

### Document: docs/en/snapshot/migration.md

# Migrate snapshot data

If the snapshot task contains multiple databases/tables, tables will be sorted **first by database name and then table name**, and they will be migrated to the target **one by one**. Only one table will be in the sync process at a time.

If the table has a single primary/unique key, the extractor will use this key as the sorting column and pull data in batches of size [pipeline] `buffer_size`, starting from the smallest value and moving upwards.

If the table does not have a sorting column, the extractor will pull all data in stream.

# Example: MySQL -> MySQL

Refer to [task templates](../../templates/mysql_to_mysql.md) and [tutorial](../tutorial/mysql_to_mysql.md)

# Parallelizer

- Redis_to_Redis: parallel_type=redis
- Others: parallel_type=snapshot

# Other configurations

- For [filter] and [router], refer to [config details](../config.md).
- Refer to task_config.ini in tests:
    - dt-tests/tests/mysql_to_mysql/snapshot
    - dt-tests/tests/pg_to_pg/snapshot
    - dt-tests/tests/mongo_to_mongo/snapshot
    - dt-tests/tests/redis_to_redis/snapshot

- Modify performance parameters if needed:
```
[extractor]
batch_size=10000

[pipeline]
buffer_size=16000
checkpoint_interval_secs=10
max_rps=10000

[sinker]
batch_size=200

[parallelizer]
parallel_size=8
```


---

### Document: docs/en/snapshot/resume.md

# Resume at breakpoint

Task progress will be recorded periodically in position.log / finished.log.

If a task interrupts, you need to restart it manually. By default, it will start from the beginning.

To avoid handling duplicate data, the task can resume at the breakpoint in position.log / finished.log.

Since resuming depends on position.log, if you have multiple tasks, **you must set up separate log directories for each task**.

## Supported
- MySQL as source
- Postgres as source
- Mongo as source

# Position Info
[position Info](../monitor/position.md)

## position.log
```
2024-10-10 04:04:08.152044 | current_position | {"type":"RdbSnapshot","db_type":"mysql","schema":"test_db","tb":"b","order_col":"id","value":"6"}
2024-10-10 04:04:08.152181 | checkpoint_position | {"type":"None"}
```

## finished.log
```
2024-10-10 04:04:07.803422 | {"type":"RdbSnapshotFinished","db_type":"mysql","schema":"test_db","tb":"a"}
2024-10-10 04:04:08.844988 | {"type":"RdbSnapshotFinished","db_type":"mysql","schema":"test_db","tb":"b"}
```

# Configurations
## Method 1: Resume from position.log & finished.log (Recommended)
```
[resumer]
resume_from_log=true
```
- after task restarts, it will resume from the breakpoints recorded in position.log and finished.log.
- note: the new task must use the same log_dir as the original task, otherwise method 1 won't take effect.
- tables in finished.log will won't be migrated.
- uncompleted tables will be migrated from the breakpoint based on position.log.
- if a table does not have a single column **primary key/unique key**, no progress info will be in position.log, but it will be in finished.log once finished.

## Method 2: Set resume config file (For ape-dts management system development)
- users may specify custom paths for resume_log_dir or resume_config_file:

```
[resumer]
resume_log_dir=./resume_logs
resume_config_file=./resume.config
```
- note: resume_log_dir and resume.config are not generated by ape-dts, but manually created by users to define custom breakpoints.
- users should write their desired breakpoint information into:
    - resume_log_dir/position.log
    - resume_log_dir/finished.log
    - resume.config
- resume.config has same contents as position.log/finished.log(log timestamp header is optional), example:

```
| current_position | {"type":"RdbSnapshot","db_type":"mysql","schema":"test_db","tb":"a","order_col":"id","value":"6"}
{"type":"RdbSnapshotFinished","db_type":"mysql","schema":"test_db","tb":"d"}
```

- if a table exists in both position.log and resume.config, position.log will be used.

# Example
- task_config.ini
```
[resumer]
resume_from_log=true
resume_log_dir=./resume_logs
resume_config_file=./resume.config
```

- ./resume.config (filled in by user)
```
{"type":"RdbSnapshotFinished","db_type":"mysql","schema":"test_db_@","tb":"finished_table_*$1"}
{"type":"RdbSnapshotFinished","db_type":"mysql","schema":"test_db_@","tb":"finished_table_*$2"}
{"type":"RdbSnapshot","db_type":"mysql","schema":"test_db_1","tb":"one_pk_no_uk","order_col":"f_0","value":"5"}
{"type":"RdbSnapshot","db_type":"mysql","schema":"test_db_1","tb":"one_pk_multi_uk","order_col":"f_0","value":"5"}
{"type":"RdbSnapshot","db_type":"mysql","schema":"test_db_@","tb":"resume_table_*$4","order_col":"p.k","value":"1"}
```

- ./resume_logs/finished.log (filled in by user)
```
2024-04-01 07:08:05.459594 | {"type":"RdbSnapshotFinished","db_type":"mysql","schema":"test_db_@","tb":"in_finished_log_table_*$1"}
2024-04-01 07:08:06.537135 | {"type":"RdbSnapshotFinished","db_type":"mysql","schema":"test_db_@","tb":"in_finished_log_table_*$2"}
```

- ./resume_logs/position.log (filled in by user)
```
2024-03-29 07:02:24.463776 | current_position | {"type":"RdbSnapshot","db_type":"mysql","schema":"test_db_@","tb":"in_position_log_table_*$1","order_col":"p.k","value":"0"}
2024-03-29 07:02:24.463777 | current_position | {"type":"RdbSnapshot","db_type":"mysql","schema":"test_db_@","tb":"in_position_log_table_*$1","order_col":"p.k","value":"1"}
```

- `test_db_@`.`finished_table_*$1`, `test_db_@`.`finished_table_*$2` are marked finished in resume.config.
- `test_db_@`.`in_finished_log_table_*$1`, `test_db_@`.`in_finished_log_table_*$2` are marked finished in finished.log.
- `test_db_1`.`one_pk_no_uk`, `test_db_1`.`one_pk_multi_uk`, `test_db_@`.`resume_table_*$4` have position info in resume.config.
- `test_db_@`.`in_position_log_table_*$1` have position info in position.log.


After task restarts, default.log:

```
2024-10-18 06:51:10.161794 - INFO - [1180981] - resumer, get resume value, schema: test_db_1, tb: one_pk_multi_uk, col: f_0, result: Some("5")
2024-10-18 06:51:11.193382 - INFO - [1180981] - resumer, get resume value, schema: test_db_1, tb: one_pk_no_uk, col: f_0, result: Some("5")
2024-10-18 06:51:12.135065 - INFO - [1180981] - resumer, check finished: schema: test_db_@, tb: finished_table_*$1, result: true
2024-10-18 06:51:12.135186 - INFO - [1180981] - resumer, check finished: schema: test_db_@, tb: finished_table_*$2, result: true
2024-10-18 06:51:12.135227 - INFO - [1180981] - resumer, check finished: schema: test_db_@, tb: in_finished_log_table_*$1, result: true
2024-10-18 06:51:12.135265 - INFO - [1180981] - resumer, check finished: schema: test_db_@, tb: in_finished_log_table_*$2, result: true
2024-10-18 06:51:12.268390 - INFO - [1180981] - resumer, get resume value, schema: test_db_@, tb: in_position_log_table_*$1, col: p.k, result: Some("1")
2024-10-18 06:51:13.390645 - INFO - [1180981] - resumer, get resume value, schema: test_db_@, tb: resume_table_*$4, col: p.k, result: Some("1")
```

## Refer to tests
- dt-tests/tests/mysql_to_mysql/snapshot/resume_test
- dt-tests/tests/pg_to_pg/snapshot/resume_test
- dt-tests/tests/mongo_to_mongo/snapshot/resume_test
---

### Document: docs/en/snapshot/review.md

# Review data

After data revision, you can review the data again based on the check results.

The check results serve as a guide for specifying the rows/scope to be reviewed, and you still need to get the current data for each row from the source database, to compare it with the target.

# Example: MySQL -> MySQL

Refer to [task templates](../../templates/mysql_to_mysql.md) and [tutorial](../tutorial/mysql_to_mysql.md)

## Note

While this configuration is similar to that of snapshot migration, the only differences are:

```
[extractor]
extract_type=check_log
check_log_dir=./dt-tests/tests/mysql_to_mysql/revise/basic_test/check_log

[sinker]
sink_type=check

[parallelizer]
parallel_type=rdb_check
```

# Other configurations

- For [router], refer to [config details](../config.md).
- Refer to task_config.ini in tests:
    - dt-tests/tests/mysql_to_mysql/review
    - dt-tests/tests/pg_to_pg/review
    - dt-tests/tests/mongo_to_mongo/review

---

### Document: docs/en/snapshot/revise.md

# Revise data

Based on the check results, you can initiate a revision task.

The check results serve as a guide for specifying the scope for revision, and you still need to get the current data for each row from the source database, to fix the data.

# Example: MySQL -> MySQL

Refer to [task templates](../../templates/mysql_to_mysql.md) and [tutorial](../tutorial/mysql_to_mysql.md)

## Note

While this configuration is similar to that of snapshot migration, the only differences are:

```
[extractor]
extract_type=check_log
check_log_dir=./dt-tests/tests/mysql_to_mysql/revise/basic_test/check_log
```

# Other configurations

- For [router], refer to [config details](../config.md).
- Refer to task_config.ini in tests:
    - dt-tests/tests/mysql_to_mysql/revise
    - dt-tests/tests/pg_to_pg/revise
    - dt-tests/tests/mongo_to_mongo/revise

---

### Document: docs/en/snapshot/tb_in_parallel.md

# Multiple Tables in Parallel for Snapshot Task

By default, when a snapshot task includes multiple tables, ape-dts migrates tables one at a time in order, sorted by **database name first, then table name**.

If you have sufficient resources (memory, CPU), you can enable parallel table migration to accelerate.

## Configuration
- with following configuration, ape-dts will migrate 4 tables at a time. When any table completes, it will sequentially select another table from the remaining ones to migrate, ensuring that 4 tables are being migrated simultaneously.

```
[runtime]
tb_parallel_size=4
```

## Difference from [parallelizer] parallel_size
- In snapshot tasks, the configuration in [parallelizer] applies to each individual table. For example, the following configuration means each table being migrated will use 8 threads to write to the target in parallel.
- While [runtime] tb_parallel_size means that 4 tables are being migrated simultaneously in the task.

```
[parallelizer]
parallel_type=snapshot
parallel_size=8
```

## Scenarios
- Snapshot migration (Source: MySQL, Postgres, MongoDB)
- Snapshot check (Source: MySQL, Postgres, MongoDB)
---

### Document: docs/en/structure/check.md

# Check structures

After structure migration, you can choose from two methods for verification. One is provided by us, and the other is an open source tool called [Liquibase](./check_liquibase.md). This document primarily focuses on the former one.

# Example: MySQL -> MySQL
Refer to [task templates](../../templates/mysql_to_mysql.md)

# Results

Based on the source structures, the check results include **miss**, **diff** and **extra**, all presented in sql. `miss.log` contains src_sqls, `diff.log` contains src_sqls and dst_sqls, and `extra.log` contains dst_sqls.

- `miss.log`
```
[("table.struct_check_test_1.not_match_miss", "CREATE TABLE `struct_check_test_1`.`not_match_miss` (`id` int(11) NOT NULL  ,`text` varchar(10) CHARACTER SET utf8 COLLATE utf8_general_ci NULL  , PRIMARY KEY (`id`)) ENGINE=InnoDB  DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci")]
key: index.struct_check_test_1.not_match_index.i6_miss, src_sql: CREATE  INDEX `i6_miss` ON `struct_check_test_1`.`not_match_index` (`index_col`) 
key: index.struct_check_test_1.not_match_index.i5_diff_name_src, src_sql: CREATE  INDEX `i5_diff_name_src` ON `struct_check_test_1`.`not_match_index` (`index_col`) 
```

- `diff.log`
```
key: index.struct_check_test_1.not_match_index.i4_diff_order, src_sql: CREATE  INDEX `i4_diff_order` ON `struct_check_test_1`.`not_match_index` (`composite_index_col2`,`composite_index_col1`,`composite_index_col3`) 
key: index.struct_check_test_1.not_match_index.i4_diff_order, dst_sql: CREATE  INDEX `i4_diff_order` ON `struct_check_test_1`.`not_match_index` (`composite_index_col3`,`composite_index_col2`,`composite_index_col1`) 
key: table.struct_check_test_1.not_match_column, src_sql: CREATE TABLE `struct_check_test_1`.`not_match_column` (`id` int(10) unsigned auto_increment NOT NULL  ,`varchar_col` varchar(255) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL  ,`char_col` char(10) CHARACTER SET utf8 COLLATE utf8_general_ci NULL  ,`text_col` text CHARACTER SET utf8 COLLATE utf8_general_ci NULL  ,`tinyint_col` tinyint(4) DEFAULT '0' NULL  ,`smallint_col` smallint(6) NULL  ,`mediumint_col` mediumint(9) NULL  ,`int_col` int(11) NULL  ,`bigint_col` bigint(20) NULL  ,`float_col` float(8,2) NULL  ,`double_col` double(16,4) NULL  ,`decimal_col` decimal(10,2) NULL  ,`date_col` date NULL  ,`datetime_col` datetime NULL  ,`timestamp_col` timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL  ,`time_col` time NULL  ,`year_col` year(4) NULL  ,`binary_col` binary(16) NULL  ,`varbinary_col` varbinary(255) NULL  ,`blob_col` blob NULL  ,`tinyblob_col` tinyblob NULL  ,`mediumblob_col` mediumblob NULL  ,`longblob_col` longblob NULL  ,`enum_col` enum('value1','value2','value3') CHARACTER SET utf8 COLLATE utf8_general_ci NULL  ,`set_col` set('option1','option2','option3') CHARACTER SET utf8 COLLATE utf8_general_ci NULL  , PRIMARY KEY (`id`)) ENGINE=InnoDB  DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci
key: table.struct_check_test_1.not_match_column, dst_sql: CREATE TABLE `struct_check_test_1`.`not_match_column` (`id` int(10) unsigned auto_increment NOT NULL  ,`char_col` char(10) CHARACTER SET utf8 COLLATE utf8_general_ci NULL  ,`text_col` text CHARACTER SET utf8 COLLATE utf8_general_ci NULL  ,`tinyint_col` tinyint(4) DEFAULT '0' NULL  ,`smallint_col` smallint(6) NULL  ,`mediumint_col` mediumint(9) NULL  ,`int_col` int(11) NULL  ,`bigint_col` bigint(20) NULL  ,`float_col` float(8,2) NULL  ,`double_col` double(16,4) NULL  ,`decimal_col` decimal(10,2) NULL  ,`datetime_col` datetime NULL  ,`timestamp_col` timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL  ,`time_col` time NULL  ,`year_col` year(4) NULL  ,`binary_col` binary(16) NULL  ,`varbinary_col` varbinary(255) NULL  ,`blob_col` blob NULL  ,`tinyblob_col` tinyblob NULL  ,`mediumblob_col` mediumblob NULL  ,`longblob_col` longblob NULL  ,`enum_col` enum('value1','value2','value3') CHARACTER SET utf8 COLLATE utf8_general_ci NULL  ,`set_col` set('option1','option2','option3') CHARACTER SET utf8 COLLATE utf8_general_ci NULL  , PRIMARY KEY (`id`)) ENGINE=InnoDB  DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci
```

- `extra.log`
```
key: index.struct_check_test_1.not_match_index.i5_diff_name_dst, dst_sql: CREATE  INDEX `i5_diff_name_dst` ON `struct_check_test_1`.`not_match_index` (`index_col`) 
```
---

### Document: docs/en/structure/check_by_liquibase.md

# Check structures by Liquibase

Liquibase is an open-source software used for managing database schema changes. You can learn more about it by visiting the [Liquibase official website](https://www.liquibase.org/) and exploring the [GitHub repository](https://github.com/liquibase/liquibase).

Since it provides comparisons for database structures, we made a docker image(apecloud/ape-dts-structure-checker:0.0.1) based on Liquibase, which can be used for check tasks after structure migration. Refer to code changes in this [GitHub repository](https://github.com/qianyiwen2019/liquibase/tree/ape_diff_tool).

# Examples

## MySQL
```
docker run \
-e URL="jdbc:mysql://host.docker.internal:3308/test_db_1?useSSL=false" \
-e USERNAME=root \
-e PASSWORD=123456 \
-e REFERENCE_URL="jdbc:mysql://host.docker.internal:3307/test_db_1?useSSL=false" \
-e REFERENCE_USERNAME=root \
-e REFERENCE_PASSWORD=123456 \
apecloud/ape-dts-structure-checker:0.0.1
```

## PG
```
docker run \
-e URL="jdbc:postgresql://host.docker.internal:5438/postgres?currentSchema=struct_check_test_1" \
-e USERNAME=postgres \
-e PASSWORD=postgres \
-e REFERENCE_URL="jdbc:postgresql://host.docker.internal:5437/postgres?currentSchema=struct_check_test_1" \
-e REFERENCE_USERNAME=postgres \
-e REFERENCE_PASSWORD=postgres \
apecloud/ape-dts-structure-checker:0.0.1
```

# Parameters

- URL: Target database url.
- USERNAME: Target username.
- PASSWORD: Target password.
- REFERENCE_URL: Source database url.
- REFERENCE_USERNAME: Source username.
- REFERENCE_PASSWORD: Source password.

# Results

```
Compared Schemas: test_db_1
Product Name: EQUAL
Product Version: EQUAL
Missing Catalog(s): NONE
Unexpected Catalog(s): NONE
Changed Catalog(s): NONE
Missing Column(s): 
     test_db_1.ape_dts_heartbeat.flushed_binlog_filename
     test_db_1.ape_dts_heartbeat.flushed_next_event_position
     test_db_1.ape_dts_heartbeat.flushed_timestamp
Unexpected Column(s): 
     test_db_1.col_has_special_character_table.col"1
Changed Column(s): NONE
Missing Foreign Key(s): NONE
Unexpected Foreign Key(s): NONE
Changed Foreign Key(s): NONE
Missing Index(s): 
     PRIMARY UNIQUE  ON test_db_1.ape_dts_heartbeat(server_id)
Unexpected Index(s): 
     PRIMARY UNIQUE  ON test_db_1.col_has_special_character_table(p:k)
     PRIMARY UNIQUE  ON test_db_1.numeric_table(f_0)
     PRIMARY UNIQUE  ON test_db_1.one_pk_multi_uk(f_0)
     PRIMARY UNIQUE  ON test_db_1.one_pk_no_uk(f_0)
     uk_1 UNIQUE  ON test_db_1.no_pk_multi_uk(f_1, f_2)
     uk_1 UNIQUE  ON test_db_1.no_pk_one_uk(f_1, f_2)
     uk_1 UNIQUE  ON test_db_1.one_pk_multi_uk(f_1, f_2)
     uk_2 UNIQUE  ON test_db_1.no_pk_multi_uk(f_3, f_4, f_5)
     uk_2 UNIQUE  ON test_db_1.one_pk_multi_uk(f_3, f_4, f_5)
     uk_3 UNIQUE  ON test_db_1.no_pk_multi_uk(f_6, f_7, f_8)
     uk_3 UNIQUE  ON test_db_1.one_pk_multi_uk(f_6, f_7, f_8)
Changed Index(s): NONE
Missing Primary Key(s): 
     PRIMARY on test_db_1.ape_dts_heartbeat(server_id)
Unexpected Primary Key(s): 
     PRIMARY on test_db_1.col_has_special_character_table(p:k)
     PRIMARY on test_db_1.numeric_table(f_0)
     PRIMARY on test_db_1.one_pk_multi_uk(f_0)
     PRIMARY on test_db_1.one_pk_no_uk(f_0)
Changed Primary Key(s): NONE
Missing Sequence(s): NONE
Unexpected Sequence(s): NONE
Changed Sequence(s): NONE
Missing Table(s): 
     ape_dts_heartbeat
Unexpected Table(s): 
     col_has_special_character_table
     no_pk_multi_uk
     no_pk_no_uk
     no_pk_one_uk
     numeric_table
     one_pk_multi_uk
     one_pk_no_uk
Changed Table(s): NONE
Missing Unique Constraint(s): NONE
Unexpected Unique Constraint(s): 
     uk_1 on no_pk_multi_uk(f_1, f_2)
     uk_1 on no_pk_one_uk(f_1, f_2)
     uk_1 on one_pk_multi_uk(f_1, f_2)
     uk_2 on no_pk_multi_uk(f_3, f_4, f_5)
     uk_2 on one_pk_multi_uk(f_3, f_4, f_5)
     uk_3 on no_pk_multi_uk(f_6, f_7, f_8)
     uk_3 on one_pk_multi_uk(f_6, f_7, f_8)
Changed Unique Constraint(s): NONE
Missing View(s): NONE
Unexpected View(s): NONE
Changed View(s): NONE
Liquibase command 'diff' was executed successfully.
```
---

### Document: docs/en/structure/migration.md

# Migrate structures

- Database: MySQL, PG.
- Migrated Objects: database(mysql), schema(pg), table, comment, index, sequence(pg), constraints.

# Example: MySQL -> MySQL

Refer to [task templates](../../templates/mysql_to_mysql.md) and [tutorial](../tutorial/mysql_to_mysql.md)

## Note

Structure migration is executed serially in a single thread. Notice the following configurations:

```
[extractor]
extract_type=struct

[sinker]
sink_type=struct
batch_size=1

[parallelizer]
parallel_type=serial
parallel_size=1
```

Failure strategy: interrupt(default), ignore.

- interrupt: If a particular migration fails, the entire task will be terminated immediately.

- ignore: If a migration fails, it will not affect the migration of other schemas, and the process will continue. However, the failure will be logged as an error.

```
[sinker]
conflict_policy=interrupt
```

# Phased migration

In a complete data migration process that includes both structure migration and data migration, the task will be divided into three stages in order to accelerate data migration:
1. Migrate table structures + primary/unique keys ( necessities for data migration);
2. Data migration;
3. Migrate indexes + constraints.

Thus, we offer 2 types of filtering:

## Migrate table structures + primary/unique keys
```
[filter]
do_structures=database,table
```

## Migrate indexes and constraints
```
[filter]
do_structures=constraint,index
```
---

### Document: docs/en/tutorial/etl_by_lua.md

# Modify data by Lua
In the following types of tasks, you can modify data by a Lua script before they were written to target.

- mysql -> mysql
- mysql -> kafka
- pg -> pg
- pg -> kafka

For details, refer to [etl by lua](../etl/lua.md)

This is a tutorial on using Lua script to edit data for mysql -> mysql task.

# Prerequisites
- [prerequisites](./prerequisites.md)

# Prepare MySQL instances

Refer to [mysql to mysql](./mysql_to_mysql.md)

# Lua script
```
cat <<EOL > /tmp/ape_dts/etl.lua
if (schema == "test_db" and tb == "tb_1" and row_type == "insert")
then
    after["value"] = 10000
end
EOL
```

# Snapshot task

## Prepare data
```
mysql -h127.0.0.1 -uroot -p123456 -P3307

CREATE DATABASE test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

```
mysql -h127.0.0.1 -uroot -p123456 -P3308

CREATE DATABASE test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=snapshot
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
db_type=mysql
sink_type=write
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled

[processor]
lua_code_file=/etl.lua

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/etl.lua:/etl.lua" \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -h127.0.0.1 -uroot -p123456 -P3308

SELECT * FROM test_db.tb_1;
```
```
+----+-------+
| id | value |
+----+-------+
|  1 | 10000 |
|  2 | 10000 |
|  3 | 10000 |
|  4 | 10000 |
+----+-------+
```

# Cdc task

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=cdc
server_id=2000
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[filter]
do_dbs=test_db
do_events=insert,update,delete

[processor]
lua_code_file=/etl.lua

[sinker]
db_type=mysql
sink_type=write
batch_size=200
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/etl.lua:/etl.lua" \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3307

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=2000000 WHERE id=2;
INSERT INTO test_db.tb_1 VALUES(5,5);
```

## Check results
```
mysql -h127.0.0.1 -uroot -p123456 -P3308

SELECT * FROM test_db.tb_1;
```
```
+----+---------+
| id | value   |
+----+---------+
|  2 | 2000000 |
|  3 |   10000 |
|  4 |   10000 |
|  5 |   10000 |
+----+---------+
```
---

### Document: docs/en/tutorial/mongo_to_mongo.md

# Migrate data from Mongo to Mongo

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/mongo_to_mongo.md) and [common configs](/docs/en/config.md) for more details.

# Prepare Mongo instances

## Source

```
docker run -d --name src-mongo \
    -p 27017:27017 \
    "$MONGO_IMAGE" --replSet rs0

-- enable and check oplog 
docker exec -it src-mongo mongosh --quiet --eval "rs.initiate()"
```

## Target

```
docker run -d --name dst-mongo \
	-e MONGO_INITDB_ROOT_USERNAME=ape_dts \
	-e MONGO_INITDB_ROOT_PASSWORD=123456 \
    -p 27018:27017 \
	"$MONGO_IMAGE"
```

# Migrate snapshot data
## Prepare data
```
docker exec -it src-mongo mongosh --quiet

use test_db;
db.tb_1.insertOne({ "name": "c", "age": "1", "_id": "1" });
db.tb_1.insertOne({ "name": "d", "age": "2", "_id": "2" });
db.tb_1.insertOne({ "name": "a", "age": "3" });
db.tb_1.insertOne({ "name": "b", "age": "4" });

db.tb_1.find();
```

```
[
  { _id: '1', name: 'c', age: '1' },
  { _id: '2', name: 'd', age: '2' },
  { _id: ObjectId("670cc7d95bace351d307453b"), name: 'a', age: '3' },
  { _id: ObjectId("670cc7d95bace351d307453c"), name: 'b', age: '4' }
]
```

## Start task
```
rm -rf /tmp/ape_dts
mkdir -p /tmp/ape_dts

cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mongo
extract_type=snapshot
url=mongodb://127.0.0.1:27017

[sinker]
db_type=mongo
sink_type=write
url=mongodb://ape_dts:123456@127.0.0.1:27018

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
docker exec -it dst-mongo mongosh \
--host localhost --port 27017 --authenticationDatabase admin -u ape_dts -p 123456 \
--eval "db = db.getSiblingDB('test_db'); db.tb_1.find()"
```

```
[
  { _id: '1', name: 'c', age: '1' },
  { _id: ObjectId("670cc7d95bace351d307453b"), name: 'a', age: '3' },
  { _id: ObjectId("670cc7d95bace351d307453c"), name: 'b', age: '4' },
  { _id: '2', name: 'd', age: '2' }
]
```

# Check data
- check the differences between target data and source data

## Prepare data
- change target table records
```
docker exec -it dst-mongo mongosh \
--host localhost --port 27017 --authenticationDatabase admin -u ape_dts -p 123456 

use test_db;
db.tb_1.deleteOne({ "_id": "1" });
db.tb_1.updateOne({ "_id" : "2" }, { "$set": { "age" : 200000 } });
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mongo
extract_type=snapshot
url=mongodb://127.0.0.1:27017

[sinker]
db_type=mongo
sink_type=check
url=mongodb://ape_dts:123456@127.0.0.1:27018

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=rdb_check
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-v "/tmp/ape_dts/check_data_task_log/:/logs/" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
- cat /tmp/ape_dts/check_data_task_log/check/miss.log
```
{"log_type":"Miss","schema":"test_db","tb":"tb_1","id_col_values":{"_id":"{\"String\":\"1\"}"},"diff_col_values":{}}
```
- cat /tmp/ape_dts/check_data_task_log/check/diff.log
```
{"log_type":"Diff","schema":"test_db","tb":"tb_1","id_col_values":{"_id":"{\"String\":\"2\"}"},"diff_col_values":{"doc":{"src":"{ \"_id\": \"2\", \"name\": \"d\", \"age\": \"2\" }","dst":"{ \"_id\": \"2\", \"name\": \"d\", \"age\": 200000 }"}}}
```

# Revise data
- revise target data based on "check data" task results

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mongo
extract_type=check_log
url=mongodb://127.0.0.1:27017
check_log_dir=./check_data_task_log

[sinker]
db_type=mongo
sink_type=write
url=mongodb://ape_dts:123456@127.0.0.1:27018

[filter]
do_events=*

[parallelizer]
parallel_type=mongo
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-v "/tmp/ape_dts/check_data_task_log/check/:/check_data_task_log/" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
docker exec -it dst-mongo mongosh \
--host localhost --port 27017 --authenticationDatabase admin -u ape_dts -p 123456 \
--eval "db = db.getSiblingDB('test_db'); db.tb_1.find()"
```

```
[
  { _id: ObjectId("670cc7d95bace351d307453b"), name: 'a', age: '3' },
  { _id: ObjectId("670cc7d95bace351d307453c"), name: 'b', age: '4' },
  { _id: '2', name: 'd', age: '2' },
  { _id: '1', name: 'c', age: '1' }
]
```

# Review data
- check if target data revised based on "check data" task results

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mongo
extract_type=check_log
url=mongodb://127.0.0.1:27017
check_log_dir=./check_data_task_log

[sinker]
db_type=mongo
sink_type=check
url=mongodb://ape_dts:123456@127.0.0.1:27018

[filter]
do_events=*

[parallelizer]
parallel_type=rdb_check
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-v "/tmp/ape_dts/check_data_task_log/check/:/check_data_task_log/" \
-v "/tmp/ape_dts/review_data_task_log/:/logs/" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
- /tmp/ape_dts/review_data_task_log/check/miss.log and /tmp/ape_dts/review_data_task_log/check/diff.log should be empty

# Cdc task

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mongo
extract_type=cdc
url=mongodb://127.0.0.1:27017
source=op_log

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=mongo
sink_type=write
url=mongodb://ape_dts:123456@127.0.0.1:27018

[parallelizer]
parallel_type=mongo
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
docker exec -it src-mongo mongosh --quiet

use test_db;
db.tb_1.deleteOne({ "_id": "1" });
db.tb_1.updateOne({ "_id" : "2" }, { "$set": { "age" : 200000 } });
db.tb_1.insertOne({ "name": "b", "age": "5" });
```

## Check results
```
docker exec -it dst-mongo mongosh \
--host localhost --port 27017 --authenticationDatabase admin -u ape_dts -p 123456 \
--eval "db = db.getSiblingDB('test_db'); db.tb_1.find()"
```

```
[
  { _id: '2', name: 'd', age: 200000 },
  { _id: ObjectId("670cc7d95bace351d307453b"), name: 'a', age: '3' },
  { _id: ObjectId("670cc7d95bace351d307453c"), name: 'b', age: '4' },
  { _id: ObjectId("670ccb84b6456ba2539bb75a"), name: 'b', age: '5' }
]
```
---

### Document: docs/en/tutorial/mysql_to_clickhouse.md

# Migrate data from MySQL to Clickhouse

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/mysql_to_clickhouse.md) and [common configs](/docs/en/config.md) for more details.

# Prepare MySQL instance
Refer to [mysql to mysql](./mysql_to_mysql.md)

# Prepare ClickHouse instance

```
docker run -d --name some-clickhouse-server \
--ulimit nofile=262144:262144 \
-p 9100:9000 \
-p 8123:8123 \
-e CLICKHOUSE_USER=admin -e CLICKHOUSE_PASSWORD=123456 \
"$CLICKHOUSE_IMAGE"
```

# Migrate structures
## Prepare source data
```
CREATE DATABASE test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
extract_type=struct
db_type=mysql
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
sink_type=struct
db_type=clickhouse
url=http://admin:123456@127.0.0.1:8123

[filter]
do_dbs=test_db

[parallelizer]
parallel_type=serial

[pipeline]
buffer_size=100
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
docker exec -it some-clickhouse-server clickhouse \
    client --user admin --password 123456

SHOW CREATE TABLE test_db.tb_1;
```

```
CREATE TABLE test_db.tb_1
(
    `id` Int32,
    `value` Nullable(Int32),
    `_ape_dts_is_deleted` Int8,
    `_ape_dts_timestamp` Int64
)
ENGINE = ReplacingMergeTree(_ape_dts_timestamp)
PRIMARY KEY id
ORDER BY id
SETTINGS index_granularity = 8192
```

# Migrate snapshot data
## Prepare source data
```
mysql -h127.0.0.1 -uroot -p123456 -P3307

INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=snapshot
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
db_type=clickhouse
sink_type=write
url=http://admin:123456@127.0.0.1:8123
batch_size=5000

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
docker exec -it some-clickhouse-server clickhouse \
    client --user admin --password 123456

SELECT * FROM test_db.tb_1 ORDER BY id;
```

```
   ┌─id─┬─value─┬─_ape_dts_is_deleted─┬─_ape_dts_timestamp─┐
1. │  1 │     1 │                   0 │    1731897789627   │
2. │  2 │     2 │                   0 │    1731897789627   │
3. │  3 │     3 │                   0 │    1731897789627   │
4. │  4 │     4 │                   0 │    1731897789627   │
   └────┴───────┴─────────────────────┴────────────────────┘
```

# Cdc task

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=cdc
server_id=2000
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=clickhouse
sink_type=write
url=http://admin:123456@127.0.0.1:8123
batch_size=5000

[parallelizer]
parallel_type=table
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3307

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=2000000 WHERE id=2;
INSERT INTO test_db.tb_1 VALUES(5,5);
```

## Check results
```
docker exec -it some-clickhouse-server clickhouse \
    client --user admin --password 123456

OPTIMIZE TABLE test_db.tb_1 FINAL;
SELECT * FROM test_db.tb_1;
```

```
   ┌─id─┬───value─┬─_ape_dts_is_deleted─┬─_ape_dts_timestamp─┐
1. │  1 │       1 │                   1 │    1731900431736   │
2. │  2 │ 2000000 │                   0 │    1731900431736   │
3. │  3 │       3 │                   0 │    1731900332526   │
4. │  4 │       4 │                   0 │    1731900332526   │
5. │  5 │       5 │                   0 │    1731900431736   │
   └────┴─────────┴─────────────────────┴────────────────────┘
```

# How it works

We convert source data into json and call http api to batch insert into ClickHouse, it is like:

curl -X POST -d @json_data 'http://localhost:8123/?query=INSERT%20INTO%test_db.tb_1%20FORMAT%20JSON' --user admin:123456

You can change the following configurations to adjust the batch data size.

```
[pipeline]
buffer_size=100000
buffer_memory_mb=200

[sinker]
batch_size=5000
```

Refer to [config](/docs/en/config.md) for other common configurations

# Column type mapping

| MySQL | ClickHouse |
| :-------- | :-------- |
| tinyint | Int8/UInt8 |
| smallint | Int16/UInt16 |
| mediumint | Int32/UInt32 |
| int | Int32/UInt32 |
| bigint | Int64/UInt64 |
| decimal | Decimal(P,S) |
| float | Float32 |
| double | Float64 |
| bit | UInt64 |
| datetime | DateTime64(6) |
| time | String |
| date | Date32 |
| year | Int32 |
| timestamp | DateTime64(6) |
| char | String |
| varchar | String |
| binary | String |
| varbinary | String |
| tinytext | String |
| text | String |
| mediumtext | String |
| longtext | String |
| tinyblob | String |
| blob | String |
| mediumblob | String |
| longblob | String |
| enum | String |
| set | String |
| json |String |

## Example
- Create a table with all supported types in MySQL

```
CREATE TABLE test_db.one_pk_no_uk ( 
   f_0 tinyint, 
   f_0_1 tinyint unsigned, 
   f_1 smallint, 
   f_1_1 smallint unsigned, 
   f_2 mediumint,
   f_2_1 mediumint unsigned, 
   f_3 int, 
   f_3_1 int unsigned, 
   f_4 bigint, 
   f_4_1 bigint unsigned, 
   f_5 decimal(10,4), 
   f_6 float(6,2), 
   f_7 double(8,3), 
   f_8 bit(64),
   f_9 datetime(6), 
   f_10 time(6), 
   f_11 date, 
   f_12 year, 
   f_13 timestamp(6) NULL, 
   f_14 char(255), 
   f_15 varchar(255), 
   f_16 binary(255), 
   f_17 varbinary(255), 
   f_18 tinytext, 
   f_19 text, 
   f_20 mediumtext, 
   f_21 longtext, 
   f_22 tinyblob, 
   f_23 blob, 
   f_24 mediumblob, 
   f_25 longblob, 
   f_26 enum('x-small','small','medium','large','x-large'), 
   f_27 set('a','b','c','d','e'), 
   f_28 json,
   PRIMARY KEY (f_0) );
```

- The generated sql to be executed in ClickHouse when migrate structures by ape_dts:
```
CREATE TABLE IF NOT EXISTS `test_db`.`one_pk_no_uk` (
   `f_0` Int8, 
   `f_0_1` Nullable(UInt8), 
   `f_1` Nullable(Int16), 
   `f_1_1` Nullable(UInt16), 
   `f_2` Nullable(Int32), 
   `f_2_1` Nullable(UInt32), 
   `f_3` Nullable(Int32), 
   `f_3_1` Nullable(UInt32), 
   `f_4` Nullable(Int64), 
   `f_4_1` Nullable(UInt64), 
   `f_5` Nullable(Decimal(10, 4)), 
   `f_6` Nullable(Float32), 
   `f_7` Nullable(Float64), 
   `f_8` Nullable(UInt64), 
   `f_9` Nullable(DateTime64(6)), 
   `f_10` Nullable(String), 
   `f_11` Nullable(Date32), 
   `f_12` Nullable(Int32), 
   `f_13` Nullable(DateTime64(6)), 
   `f_14` Nullable(String), 
   `f_15` Nullable(String), 
   `f_16` Nullable(String), 
   `f_17` Nullable(String), 
   `f_18` Nullable(String), 
   `f_19` Nullable(String), 
   `f_20` Nullable(String), 
   `f_21` Nullable(String), 
   `f_22` Nullable(String), 
   `f_23` Nullable(String), 
   `f_24` Nullable(String), 
   `f_25` Nullable(String), 
   `f_26` Nullable(String), 
   `f_27` Nullable(String), 
   `f_28` Nullable(String), 
   `_ape_dts_is_deleted` Int8, 
   `_ape_dts_timestamp` Int64
   ) ENGINE = ReplacingMergeTree(`_ape_dts_timestamp`) PRIMARY KEY (`f_0`) 
   ORDER BY (`f_0`)
```

# DDL during CDC is NOT supported yet
Currently, DDL events are ignored, we may support this in future.
---

### Document: docs/en/tutorial/mysql_to_doris.md

# Migrate data from MySQL to Doris

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/mysql_to_doris.md) and [common configs](/docs/en/config.md) for more details.

# Prepare MySQL instance
Refer to [mysql to mysql](./mysql_to_mysql.md)

# Prepare Doris instance
```
docker run -itd --name some-doris \
-p 9030:9030 \
-p 8030:8030 \
-p 8040:8040 \
"$DORIS_IMAGE"
```

# Migrate structures
## Prepare source data
```
mysql -h127.0.0.1 -uroot -p123456 -P3307

CREATE DATABASE test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
extract_type=struct
db_type=mysql
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
url=mysql://root:@127.0.0.1:9030
sink_type=struct
db_type=doris

[filter]
do_dbs=test_db

[parallelizer]
parallel_type=serial

[pipeline]
buffer_size=100
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="Doris > "

SHOW CREATE TABLE test_db.tb_1;
```

```
CREATE TABLE `tb_1` (
  `id` INT NOT NULL,
  `value` INT NULL
) ENGINE=OLAP
UNIQUE KEY(`id`)
COMMENT 'OLAP'
DISTRIBUTED BY HASH(`id`) BUCKETS 10
PROPERTIES (
"replication_allocation" = "tag.location.default: 1",
"min_load_replica_num" = "-1",
"is_being_synced" = "false",
"storage_medium" = "hdd",
"storage_format" = "V2",
"enable_unique_key_merge_on_write" = "true",
"light_schema_change" = "true",
"disable_auto_compaction" = "false",
"enable_single_replica_compaction" = "false",
"group_commit_interval_ms" = "10000",
"group_commit_data_bytes" = "134217728"
);
```

# Migrate snapshot data
## Prepare source data
```
mysql -h127.0.0.1 -uroot -p123456 -P3307

INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=snapshot
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
db_type=doris
sink_type=write
url=mysql://root:@127.0.0.1:9030
stream_load_url=mysql://root:@127.0.0.1:8040
batch_size=5000

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="Doris > "

SELECT * FROM test_db.tb_1;
```

```
+------+-------+
| id   | value |
+------+-------+
|    1 |     1 |
|    2 |     2 |
|    3 |     3 |
|    4 |     4 |
+------+-------+
```

# Cdc task

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=cdc
server_id=2000
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=doris
sink_type=write
url=mysql://root:@127.0.0.1:9030
stream_load_url=mysql://root:@127.0.0.1:8040
batch_size=5000

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3307

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=2000000 WHERE id=2;
INSERT INTO test_db.tb_1 VALUES(5,5);
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="Doris > "

SELECT * FROM test_db.tb_1;
```

```
+------+---------+
| id   | value   |
+------+---------+
|    2 | 2000000 |
|    3 |       3 |
|    4 |       4 |
|    5 |       5 |
+------+---------+
```

# How it works

We use [Stream Load](https://doris.apache.org/docs/1.2/data-operate/import/import-way/stream-load-manual) to import data from MySQL. You need to configure url (query metadata) and stream_load_url (specify Stream Load port and user info).

When importing data into Doris by Stream Load, you need to avoid frequent small-batch imports, as this may cause throttle errors in Doris. This can be resolved by configuring batch_sink_interval_secs, refer to [task templates](/docs/templates/mysql_to_doris.md). Usually, only CDC tasks need to configure batch_sink_interval_secs.

Stream Load allows importing up to 10GB of data in a single load. You can change the following configurations to adjust the batch data size.

```
[pipeline]
buffer_size=100000
buffer_memory_mb=200

[sinker]
batch_size=5000
```

Refer to [config](/docs/en/config.md) for other common configurations

# Data type mapping

| MySQL | Doris |
| :-------- | :-------- |
| tinyint | TINYINT |
| tinyint unsigned | SMALLINT |
| smallint | SMALLINT |
| smallint unsigned | INT |
| mediumint | INT |
| mediumint unsigned | BIGINT |
| int | INT |
| int unsigned | BIGINT |
| bigint | BIGINT |
| bigint unsigned | LARGEINT |
| decimal | DECIMAL |
| float | FLOAT |
| double | DOUBLE |
| bit | BIGINT |
| datetime | DATETIME |
| time | VARCHAR |
| date | DATE |
| year | INT |
| timestamp | DATETIME |
| char | CHAR |
| varchar | VARCHAR |
| binary | STRING |
| varbinary | STRING |
| tinytext | STRING |
| text | STRING |
| mediumtext | STRING |
| longtext | STRING |
| tinyblob | STRING |
| blob | STRING |
| mediumblob | STRING |
| longblob | STRING |
| enum | VARCHAR |
| set | VARCHAR |
| json | JSON |

## Example
- Create a table in MySQL

```
CREATE TABLE test_db.one_pk_no_uk ( 
    f_0 tinyint, 
    f_0_1 tinyint unsigned, 
    f_1 smallint, 
    f_1_1 smallint unsigned, 
    f_2 mediumint,
    f_2_1 mediumint unsigned, 
    f_3 int, 
    f_3_1 int unsigned, 
    f_4 bigint, 
    f_4_1 bigint unsigned, 
    f_5 decimal(10,4), 
    f_6 float(6,2), 
    f_7 double(8,3), 
    f_8 bit(64),
    f_9 datetime(6), 
    f_10 time(6), 
    f_11 date, 
    f_12 year, 
    f_13 timestamp(6) NULL, 
    f_14 char(255), 
    f_15 varchar(255), 
    f_16 binary(255), 
    f_17 varbinary(255), 
    f_18 tinytext, 
    f_19 text, 
    f_20 mediumtext, 
    f_21 longtext, 
    f_22 tinyblob, 
    f_23 blob, 
    f_24 mediumblob, 
    f_25 longblob, 
    f_26 enum('x-small','small','medium','large','x-large'), 
    f_27 set('a','b','c','d','e'), 
    f_28 json,
    PRIMARY KEY (f_0) );
```

- The generated sql to be executed in Doris when migrate structures by ape_dts:
```
CREATE TABLE IF NOT EXISTS `test_db`.`one_pk_no_uk` (
  `f_0` TINYINT NOT NULL, 
  `f_0_1` SMALLINT, 
  `f_1` SMALLINT, 
  `f_1_1` INT, 
  `f_2` INT, 
  `f_2_1` BIGINT, 
  `f_3` INT, 
  `f_3_1` BIGINT, 
  `f_4` BIGINT, 
  `f_4_1` LARGEINT, 
  `f_5` DECIMAL(10, 4), 
  `f_6` FLOAT, 
  `f_7` DOUBLE, 
  `f_8` BIGINT, 
  `f_9` DATETIME(6), 
  `f_10` VARCHAR(255), 
  `f_11` DATE, 
  `f_12` INT, 
  `f_13` DATETIME(6), 
  `f_14` CHAR(255), 
  `f_15` VARCHAR(255), 
  `f_16` STRING, 
  `f_17` STRING, 
  `f_18` STRING, 
  `f_19` STRING, 
  `f_20` STRING, 
  `f_21` STRING, 
  `f_22` STRING, 
  `f_23` STRING, 
  `f_24` STRING, 
  `f_25` STRING, 
  `f_26` VARCHAR(255), 
  `f_27` VARCHAR(255), 
  `f_28` JSON
) UNIQUE KEY (`f_0`) DISTRIBUTED BY HASH(`f_0`) PROPERTIES ("replication_num" = "1")
```

# Supported versions

We've tested on apache/doris:doris-all-in-one-2.1.0, refer to [tests](/dt-tests/tests/mysql_to_doris/)

# DDL during CDC is NOT supported yet
Currently, DDL events are ignored, we may support this in future.
---

### Document: docs/en/tutorial/mysql_to_http_server_consumer.md

# Start as HTTP server and extract MySQL data

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/rdb_to_http_server.md) and [common configs](/docs/en/config.md) for more details.

- Refer to [Start ape_dts as HTTP server to provide data to consumers](/docs/en/consumer/http_consumer.md) for task description.

# Prepare MySQL instance
Refer to [mysql to mysql](./mysql_to_mysql.md)

# CDC task

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=cdc
server_id=2000
url=mysql://root:123456@host.docker.internal:3307?ssl-mode=disabled

[sinker]
sink_type=dummy

[parallelizer]
parallel_type=serial
parallel_size=1

[filter]
do_dbs=test_db,test_db_2
do_events=insert,update,delete
do_ddls=*

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
pipeline_type=http_server
http_host=0.0.0.0
http_port=10231
with_field_defs=true
EOL
```

```
docker run --rm \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-p 10231:10231 \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Make changes in MySQL
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3307

CREATE DATABASE test_db_2;
CREATE TABLE test_db_2.tb_2(id int, value int, primary key(id));
INSERT INTO test_db_2.tb_2 VALUES(1,1);
UPDATE test_db_2.tb_2 SET value=100000 WHERE id=1;
DELETE FROM test_db_2.tb_2;
```

# Start consumer

[python / golang consumer demo](https://github.com/apecloud/ape_dts_consumer_demo)
---

### Document: docs/en/tutorial/mysql_to_kafka_consumer.md

# Send MySQL data to Kafka

Refer to [Send data to Kafka](/docs/en/consumer/kafka_consumer.md) for consumers.

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/rdb_to_kafka.md) and [common configs](/docs/en/config.md) for more details.

# Prepare MySQL instance
Refer to [mysql to mysql](./mysql_to_mysql.md)

# Prepare Kafka instance
- start zookeeper
```
rm -rf /tmp/ape_dts/kafka/zookeeper_data
mkdir -p /tmp/ape_dts/kafka/zookeeper_data

docker run --name some-zookeeper \
-p 2181:2181 \
-v "/tmp/ape_dts/kafka/zookeeper_data:/bitnami" \
-e ALLOW_ANONYMOUS_LOGIN=yes \
-d "$ZOOKEEPER_IMAGE"
```

- start kafka
```
rm -rf /tmp/ape_dts/kafka/kafka_data
mkdir -p /tmp/ape_dts/kafka/kafka_data

docker run --name some-kafka \
-p 9092:9092 \
-p 9093:9093 \
-v "/tmp/ape_dts/kafka/kafka_data:/bitnami/kafka" \
-e KAFKA_CFG_ZOOKEEPER_CONNECT=host.docker.internal:2181 \
-e ALLOW_PLAINTEXT_LISTENER=yes \
-e KAFKA_CFG_LISTENER_SECURITY_PROTOCOL_MAP=CLIENT:PLAINTEXT,EXTERNAL:PLAINTEXT \
-e KAFKA_CFG_LISTENERS=CLIENT://:9092,EXTERNAL://:9093 \
-e KAFKA_CFG_ADVERTISED_LISTENERS=CLIENT://127.0.0.1:9092,EXTERNAL://127.0.0.1:9093 \
-e KAFKA_CFG_INTER_BROKER_LISTENER_NAME=CLIENT \
-d "$KAFKA_IMAGE"
```

- create test topic
```
docker exec -it some-kafka /opt/bitnami/kafka/bin/kafka-topics.sh --create --topic test --bootstrap-server localhost:9093
```

# Send Snapshot data to Kafka
## Prepare data
```
mysql -h127.0.0.1 -uroot -p123456 -P3307

CREATE DATABASE test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=snapshot
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
db_type=kafka
sink_type=write
url=127.0.0.1:9093
with_field_defs=true

[filter]
do_dbs=test_db
do_events=insert

[router]
topic_map=*.*:test

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

# Send CDC data to Kafka
## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=cdc
server_id=2000
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[filter]
do_dbs=test_db,test_db_2
do_events=insert,update,delete
do_ddls=*

[router]
topic_map=*.*:test

[sinker]
db_type=kafka
sink_type=write
url=127.0.0.1:9093
with_field_defs=true

[parallelizer]
parallel_type=serial
parallel_size=1

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Make changes in MySQL
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3307

CREATE DATABASE test_db_2;
CREATE TABLE test_db_2.tb_2(id int, value int, primary key(id));
INSERT INTO test_db_2.tb_2 VALUES(1,1);
UPDATE test_db_2.tb_2 SET value=100000 WHERE id=1;
DELETE FROM test_db_2.tb_2;
```

# Run Kafka consumer demo

[python / golang consumer demo](https://github.com/apecloud/ape_dts_consumer_demo)
---

### Document: docs/en/tutorial/mysql_to_mysql.md

# Migrate data from MySQL to MySQL

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/mysql_to_mysql.md) and [common configs](/docs/en/config.md) for more details.

# Prepare MySQL instances

## Source

```
docker run -d --name some-mysql-1 \
--platform linux/x86_64 \
-it \
-p 3307:3306 -e MYSQL_ROOT_PASSWORD="123456" \
 "$MYSQL_IMAGE" --lower_case_table_names=1 --character-set-server=utf8 --collation-server=utf8_general_ci \
 --datadir=/var/lib/mysql \
 --user=mysql \
 --server_id=1 \
 --log_bin=/var/lib/mysql/mysql-bin.log \
 --max_binlog_size=100M \
 --gtid_mode=ON \
 --enforce_gtid_consistency=ON \
 --binlog_format=ROW \
 --sql_mode=NO_ENGINE_SUBSTITUTION \
 --default_time_zone=+08:00
```

## Target

```
docker run -d --name some-mysql-2 \
--platform linux/x86_64 \
-it \
-p 3308:3306 -e MYSQL_ROOT_PASSWORD="123456" \
 "$MYSQL_IMAGE" --lower_case_table_names=1 --character-set-server=utf8 --collation-server=utf8_general_ci \
 --datadir=/var/lib/mysql \
 --user=mysql \
 --server_id=1 \
 --log_bin=/var/lib/mysql/mysql-bin.log \
 --max_binlog_size=100M \
 --gtid_mode=ON \
 --enforce_gtid_consistency=ON \
 --binlog_format=ROW \
 --sql_mode=NO_ENGINE_SUBSTITUTION \
 --default_time_zone=+07:00 
```

# Migrate structures

## Prepare data
```
mysql -h127.0.0.1 -uroot -p123456 -P3307

CREATE DATABASE test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
```

## Start task
```
rm -rf /tmp/ape_dts
mkdir -p /tmp/ape_dts

cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
extract_type=struct
db_type=mysql
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
sink_type=struct
db_type=mysql
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled

[filter]
do_dbs=test_db

[parallelizer]
parallel_type=serial

[pipeline]
buffer_size=100
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3308

SHOW TABLES IN test_db;
```

```
+-------------------+
| Tables_in_test_db |
+-------------------+
| tb_1              |
+-------------------+
```

# Migrate snapshot data
## Prepare data
```
mysql -h127.0.0.1 -uroot -p123456 -P3307

INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=snapshot
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
db_type=mysql
sink_type=write
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3308

SELECT * FROM test_db.tb_1;
```

```
+----+-------+
| id | value |
+----+-------+
|  1 |     1 |
|  2 |     2 |
|  3 |     3 |
|  4 |     4 |
+----+-------+
```

# Check data
- check the differences between target data and source data

## Prepare data
- change target table records
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3308

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=1 WHERE id=2;
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=snapshot
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
db_type=mysql
sink_type=check
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=rdb_check
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-v "/tmp/ape_dts/check_data_task_log/:/logs/" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
- cat /tmp/ape_dts/check_data_task_log/check/miss.log
```
{"log_type":"Miss","schema":"test_db","tb":"tb_1","id_col_values":{"id":"1"},"diff_col_values":{}}
```
- cat /tmp/ape_dts/check_data_task_log/check/diff.log
```
{"log_type":"Diff","schema":"test_db","tb":"tb_1","id_col_values":{"id":"2"},"diff_col_values":{"value":{"src":"2","dst":"1"}}}
```

# Revise data
- revise target data based on "check data" task results

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=check_log
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled
check_log_dir=./check_data_task_log

[sinker]
db_type=mysql
sink_type=write
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled

[filter]
do_events=*

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-v "/tmp/ape_dts/check_data_task_log/check/:/check_data_task_log/" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3308

SELECT * FROM test_db.tb_1;
```

```
+----+-------+
| id | value |
+----+-------+
|  1 |     1 |
|  2 |     2 |
|  3 |     3 |
|  4 |     4 |
+----+-------+
```

# Review data
- check if target data revised based on "check data" task results

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=check_log
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled
check_log_dir=./check_data_task_log

[sinker]
db_type=mysql
sink_type=check
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled

[filter]
do_events=*

[parallelizer]
parallel_type=rdb_check
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-v "/tmp/ape_dts/check_data_task_log/check/:/check_data_task_log/" \
-v "/tmp/ape_dts/review_data_task_log/:/logs/" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
- /tmp/ape_dts/review_data_task_log/check/miss.log and /tmp/ape_dts/review_data_task_log/check/diff.log should be empty

# Cdc task

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=cdc
server_id=2000
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=mysql
sink_type=write
batch_size=200
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3307

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=2000000 WHERE id=2;
INSERT INTO test_db.tb_1 VALUES(5,5);
```

## Check results
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3308

SELECT * FROM test_db.tb_1;
```

```
+----+---------+
| id | value   |
+----+---------+
|  2 | 2000000 |
|  3 |       3 |
|  4 |       4 |
|  5 |       5 |
+----+---------+
```

# Cdc task with gtid
## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=cdc
server_id=2000
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled
gtid_enabled=true
gtid_set=

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=mysql
sink_type=write
batch_size=200
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3307

DELETE FROM test_db.tb_1 WHERE id=3;
UPDATE test_db.tb_1 SET value=2000000 WHERE id=4;
INSERT INTO test_db.tb_1 VALUES(6,6);
```

## Check results
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3308

SELECT * FROM test_db.tb_1;
```

```
+----+---------+
| id | value   |
+----+---------+
|  2 | 2000000 |
|  4 | 2000000 |
|  5 |       5 |
|  6 |       6 |
+----+---------+
```

# CDC task with ddl capture

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=cdc
server_id=2000
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[filter]
do_dbs=test_db
do_events=insert,update,delete
do_ddls=create_database,drop_database,alter_database,create_table,alter_table,drop_table,create_index,drop_index,truncate_table,rename_table

[sinker]
db_type=mysql
sink_type=write
batch_size=200
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Do ddls in source
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3307

CREATE TABLE test_db.tb_2(id int, value int, primary key(id));
INSERT INTO test_db.tb_2 VALUES(1,1);
```

## Check results
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3308

SELECT * FROM test_db.tb_2;
```

```
+----+-------+
| id | value |
+----+-------+
|  1 |     1 |
+----+-------+
```
---

### Document: docs/en/tutorial/mysql_to_starrocks.md

# Migrate data from MySQL to StarRocks

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/mysql_to_starrocks.md) and [common configs](/docs/en/config.md) for more details.

# Prepare MySQL instance
Refer to [mysql to mysql](./mysql_to_mysql.md)

# Prepare StarRocks instance
- tested versions: 2.5.4 to 3.2.11

```
docker run -itd --name some-starrocks \
-p 9030:9030 \
-p 8030:8030 \
-p 8040:8040 \
"$STARROCKS_IMAGE"
```

# Migrate structures
## Prepare source data
```
mysql -h127.0.0.1 -uroot -p123456 -P3307

CREATE DATABASE test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
extract_type=struct
db_type=mysql
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
url=mysql://root:@127.0.0.1:9030
sink_type=struct
db_type=starrocks

[filter]
do_dbs=test_db

[parallelizer]
parallel_type=serial

[pipeline]
buffer_size=100
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="StarRocks > "

SHOW CREATE TABLE test_db.tb_1;
```

```
CREATE TABLE `tb_1` (
  `id` int(11) NOT NULL COMMENT "",
  `value` int(11) NULL COMMENT "",
  `_ape_dts_is_deleted` boolean NULL COMMENT "",
  `_ape_dts_timestamp` bigint(20) NULL COMMENT ""
) ENGINE=OLAP 
PRIMARY KEY(`id`)
DISTRIBUTED BY HASH(`id`)
PROPERTIES (
"replication_num" = "1",
"in_memory" = "false",
"enable_persistent_index" = "true",
"replicated_storage" = "true",
"compression" = "LZ4"
);
```

# Migrate snapshot data
## Prepare source data
```
mysql -h127.0.0.1 -uroot -p123456 -P3307

INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=snapshot
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
db_type=starrocks
sink_type=write
url=mysql://root:@127.0.0.1:9030
stream_load_url=mysql://root:@127.0.0.1:8040
batch_size=5000

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="StarRocks > "

SELECT * FROM test_db.tb_1;
```

```
+------+-------+---------------------+--------------------+
| id   | value | _ape_dts_is_deleted | _ape_dts_timestamp |
+------+-------+---------------------+--------------------+
|    1 |     1 |                NULL |    1731665154965   |
|    2 |     2 |                NULL |    1731665159858   |
|    3 |     3 |                NULL |    1731665159880   |
|    4 |     4 |                NULL |    1731665159880   |
+------+-------+---------------------+--------------------+
```

# Cdc task with hard delete (NOT recommended)

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=cdc
server_id=2000
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=starrocks
sink_type=write
url=mysql://root:@127.0.0.1:9030
stream_load_url=mysql://root:@127.0.0.1:8040
hard_delete=true
batch_size=5000

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3307

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=2000000 WHERE id=2;
INSERT INTO test_db.tb_1 VALUES(5,5);
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="StarRocks > "

SELECT * FROM test_db.tb_1;
```

```
+------+---------+---------------------+--------------------+
| id   | value   | _ape_dts_is_deleted | _ape_dts_timestamp |
+------+---------+---------------------+--------------------+
|    2 | 2000000 |                NULL |    1731665679461   |
|    3 |       3 |                NULL |    1731665609225   |
|    4 |       4 |                NULL |    1731665609236   |
|    5 |       5 |                NULL |    1731665679572   |
+------+---------+---------------------+--------------------+
```

# Cdc task with soft delete (recommended)
## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=mysql
extract_type=cdc
server_id=2000
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=starrocks
sink_type=write
url=mysql://root:@127.0.0.1:9030
stream_load_url=mysql://root:@127.0.0.1:8040
batch_size=5000

[parallelizer]
parallel_type=table
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
mysql -h127.0.0.1 -uroot -p123456 -uroot -P3307

DELETE FROM test_db.tb_1 WHERE id=3;
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="StarRocks > "

SELECT * FROM test_db.tb_1;
```

```
+------+---------+---------------------+--------------------+
| id   | value   | _ape_dts_is_deleted | _ape_dts_timestamp |
+------+---------+---------------------+--------------------+
|    2 | 2000000 |                NULL |    1731665679461   |
|    3 |       3 |                   1 |    1731665747381   |
|    4 |       4 |                NULL |    1731665609236   |
|    5 |       5 |                NULL |    1731665679572   |
+------+---------+---------------------+--------------------+
```

# How it works

We use [Stream Load](https://docs.starrocks.io/docs/loading/Stream_Load_transaction_interface/) to import data from MySQL. You need to configure url (query metadata) and stream_load_url (specify Stream Load port and user info).

When importing data into StarRocks by Stream Load, you need to avoid frequent small-batch imports, as this may cause throttle errors in StarRocks. This can be resolved by configuring batch_sink_interval_secs, refer to [task templates](/docs/templates/mysql_to_starrocks.md). Usually, only CDC tasks need to configure batch_sink_interval_secs.

Stream Load allows importing up to 10GB of data in a single load. You can change the following configurations to adjust the batch data size.

```
[pipeline]
buffer_size=100000
buffer_memory_mb=200

[sinker]
batch_size=5000
```

Refer to [config](/docs/en/config.md) for other common configurations

# Data type mapping

| MySQL | StarRocks |
| :-------- | :-------- |
| tinyint | TINYINT |
| tinyint unsigned | SMALLINT |
| smallint | SMALLINT |
| smallint unsigned | INT |
| mediumint | INT |
| mediumint unsigned | BIGINT |
| int | INT |
| int unsigned | BIGINT |
| bigint | BIGINT |
| bigint unsigned | LARGEINT |
| decimal | DECIMAL |
| float | FLOAT |
| double | DOUBLE |
| bit | BIGINT |
| datetime | DATETIME |
| time | VARCHAR |
| date | DATE |
| year | INT |
| timestamp | DATETIME |
| char | CHAR |
| varchar | VARCHAR |
| binary | VARBINARY |
| varbinary | VARBINARY |
| tinytext | STRING |
| text | STRING |
| mediumtext | STRING |
| longtext | STRING |
| tinyblob | VARBINARY |
| blob | VARBINARY |
| mediumblob | VARBINARY |
| longblob | VARBINARY |
| enum | VARCHAR |
| set | VARCHAR |
| json | JSON/STRING |

## Example
- Create a table in MySQL

```
CREATE TABLE test_db.one_pk_no_uk ( 
    f_0 tinyint, 
    f_0_1 tinyint unsigned, 
    f_1 smallint, 
    f_1_1 smallint unsigned, 
    f_2 mediumint,
    f_2_1 mediumint unsigned, 
    f_3 int, 
    f_3_1 int unsigned, 
    f_4 bigint, 
    f_4_1 bigint unsigned, 
    f_5 decimal(10,4), 
    f_6 float(6,2), 
    f_7 double(8,3), 
    f_8 bit(64),
    f_9 datetime(6), 
    f_10 time(6), 
    f_11 date, 
    f_12 year, 
    f_13 timestamp(6) NULL, 
    f_14 char(255), 
    f_15 varchar(255), 
    f_16 binary(255), 
    f_17 varbinary(255), 
    f_18 tinytext, 
    f_19 text, 
    f_20 mediumtext, 
    f_21 longtext, 
    f_22 tinyblob, 
    f_23 blob, 
    f_24 mediumblob, 
    f_25 longblob, 
    f_26 enum('x-small','small','medium','large','x-large'), 
    f_27 set('a','b','c','d','e'), 
    f_28 json,
    PRIMARY KEY (f_0) );
```

- The generated sql to be executed in StarRocks when migrate structures by ape_dts:
```
CREATE TABLE IF NOT EXISTS `test_db`.`one_pk_no_uk` (
    `f_0` TINYINT NOT NULL,
    `f_0_1` SMALLINT,
    `f_1` SMALLINT,
    `f_1_1` INT,
    `f_2` INT,
    `f_2_1` BIGINT,
    `f_3` INT,
    `f_3_1` BIGINT,
    `f_4` BIGINT,
    `f_4_1` LARGEINT,
    `f_5` DECIMAL(10, 4),
    `f_6` FLOAT,
    `f_7` DOUBLE,
    `f_8` BIGINT,
    `f_9` DATETIME,
    `f_10` VARCHAR(255),
    `f_11` DATE,
    `f_12` INT,
    `f_13` DATETIME,
    `f_14` CHAR(255),
    `f_15` VARCHAR(255),
    `f_16` VARBINARY,
    `f_17` VARBINARY,
    `f_18` STRING,
    `f_19` STRING,
    `f_20` STRING,
    `f_21` STRING,
    `f_22` VARBINARY,
    `f_23` VARBINARY,
    `f_24` VARBINARY,
    `f_25` VARBINARY,
    `f_26` VARCHAR(255),
    `f_27` VARCHAR(255),
    `f_28` JSON,
    `_ape_dts_is_deleted` BOOLEAN,
    `_ape_dts_timestamp` BIGINT
) PRIMARY KEY (`f_0`) DISTRIBUTED BY HASH(`f_0`);
```

# Soft delete or Hard delete 
Due to the poor performance of StarRocks in handling delete operations, hard delete is always NOT recommended.

Soft delete prerequisites: 
- target table must have a `_ape_dts_is_deleted` column.

Hard delete prerequisites: 
- `[parallelizer] parallel_type` must be `rdb_merge`.

# Supported versions

We've tested on StarRocks 2.5.4 and 3.2.11, refer to [tests](/dt-tests/tests/mysql_to_starrocks/)

For 2.5.4, the stream_load_url should use be_http_port instead of fe_http_port.

# DDL during CDC is NOT supported yet
Currently, DDL events are ignored, we may support this in future.
---

### Document: docs/en/tutorial/mysql_to_tidb.md

# Migrate data from MySQL to TiDB

- TiDB's protocol is compatible with MySQL, ape_dts uses the same method for `MySQL -> MySQL` to achieve `MySQL -> TiDB`.

- Therefore, this article only provides a simple demo for structure migration. For other task types, please refer to [MySQL -> MySQL](./mysql_to_mysql.md)


# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/mysql_to_mysql.md) and [common configs](/docs/en/config.md) for more details.

# Prepare MySQL instance
Refer to [MySQL -> MySQL](./mysql_to_mysql.md)

# Prepare TiDB instance

- Start instance
```
docker run --name some-tidb -d \
-v /tmp/tidb/data:/tmp/tidb \
-p 4000:4000 -p 10080:10080 \
pingcap/tidb:v7.1.6
```

- Create user
```
mysql -h 127.0.0.1 -P 4000 -u root -D test --prompt="tidb> "

CREATE DATABASE demo CHARACTER SET utf8 COLLATE utf8_general_ci;
CREATE USER 'demo'@'%' IDENTIFIED BY '123456';
GRANT ALL PRIVILEGES ON *.* TO 'demo'@'%';
FLUSH PRIVILEGES;
```

# Migrate structures
## Prepare source data
```
mysql -h127.0.0.1 -uroot -p123456 -P3307

CREATE DATABASE test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
extract_type=struct
db_type=mysql
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled

[sinker]
url=mysql://demo:123456@127.0.0.1:4000?ssl-mode=disabled
sink_type=struct
db_type=tidb

[filter]
do_dbs=test_db

[parallelizer]
parallel_type=serial

[pipeline]
buffer_size=100
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -h127.0.0.1 -udemo -p123456 -P4000

SHOW CREATE TABLE test_db.tb_1;
```

```
CREATE TABLE `tb_1` (
  `id` int(11) NOT NULL,
  `value` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`) /*T![clustered_index] CLUSTERED */
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_general_ci
```

# Differences with `MySQL -> MySQL`
- For `MySQL -> TiDB` tasks, the only difference in config is:

```
[sinker]
db_type=tidb
```

- Please note that the charsets, collations, and data types supported by [TiDB](https://docs.pingcap.com/zh/tidb/stable/data-type-overview) are only a subset of those in MySQL. If you are migrating from MySQL to TiDB, make sure the data is within the supported range.
---

### Document: docs/en/tutorial/pg_to_clickhouse.md

# Migrate data from Postgres to ClickHouse

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/pg_to_clickhouse.md) and [common configs](/docs/en/config.md) for more details.

# Prepare Postgres instance
Refer to [pg to pg](./pg_to_pg.md)

# Prepare ClickHouse instance

```
docker run -d --name some-clickhouse-server \
--ulimit nofile=262144:262144 \
-p 9100:9000 \
-p 8123:8123 \
-e CLICKHOUSE_USER=admin -e CLICKHOUSE_PASSWORD=123456 \
"$CLICKHOUSE_IMAGE"
```

# Migrate structures
## Prepare source data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

CREATE SCHEMA test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
extract_type=struct
db_type=pg
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
sink_type=struct
db_type=clickhouse
url=http://admin:123456@127.0.0.1:8123

[filter]
do_dbs=test_db

[parallelizer]
parallel_type=serial

[pipeline]
buffer_size=100
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
docker exec -it some-clickhouse-server clickhouse \
    client --user admin --password 123456

SHOW CREATE TABLE test_db.tb_1;
```

```
CREATE TABLE test_db.tb_1
(
    `id` Int32,
    `value` Nullable(Int32),
    `_ape_dts_is_deleted` Int8,
    `_ape_dts_timestamp` Int64
)
ENGINE = ReplacingMergeTree(_ape_dts_timestamp)
PRIMARY KEY id
ORDER BY id
SETTINGS index_granularity = 8192

```

# Migrate snapshot data
## Prepare source data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=snapshot
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
db_type=clickhouse
sink_type=write
url=http://admin:123456@127.0.0.1:8123
batch_size=5000

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
docker exec -it some-clickhouse-server clickhouse \
    client --user admin --password 123456

SELECT * FROM test_db.tb_1;
```

```
┌─id─┬─value─┬─_ape_dts_is_deleted─┬─_ape_dts_timestamp─┐
│  1 │     1 │                   0 │      1736500603659 │
│  2 │     2 │                   0 │      1736500603659 │
│  3 │     3 │                   0 │      1736500603659 │
│  4 │     4 │                   0 │      1736500603659 │
└────┴───────┴─────────────────────┴────────────────────┘
```

# Cdc task

## Drop replication slot if exists
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

SELECT pg_drop_replication_slot('ape_test') FROM pg_replication_slots WHERE slot_name = 'ape_test';
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=cdc
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
slot_name=ape_test

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=clickhouse
sink_type=write
url=http://admin:123456@127.0.0.1:8123
batch_size=5000

[parallelizer]
parallel_type=table
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=2000000 WHERE id=2;
INSERT INTO test_db.tb_1 VALUES(5,5);
```

## Check results
```
docker exec -it some-clickhouse-server clickhouse \
    client --user admin --password 123456

OPTIMIZE TABLE test_db.tb_1 FINAL;
SELECT * FROM test_db.tb_1;
```

```
┌─id─┬───value─┬─_ape_dts_is_deleted─┬─_ape_dts_timestamp─┐
│  1 │    ᴺᵁᴸᴸ │                   1 │      1736500859060 │
│  2 │ 2000000 │                   0 │      1736500859060 │
│  3 │       3 │                   0 │      1736500603659 │
│  4 │       4 │                   0 │      1736500603659 │
│  5 │       5 │                   0 │      1736500859060 │
└────┴─────────┴─────────────────────┴────────────────────┘
```

# How it works

We convert source data into json and call http api to batch insert into ClickHouse, it is like:

curl -X POST -d @json_data 'http://localhost:8123/?query=INSERT%20INTO%test_db.tb_1%20FORMAT%20JSON' --user admin:123456

You can change the following configurations to adjust the batch data size.

```
[pipeline]
buffer_size=100000
buffer_memory_mb=200

[sinker]
batch_size=5000
```

Refer to [config](/docs/en/config.md) for other common configurations

# Data type mapping
- Create a table in Postgres

```
CREATE SCHEMA test_db;

CREATE TABLE test_db_1.full_column_type (
    id serial PRIMARY KEY, 

    -- char
    char_col char,
    char_col_2 char(255),
    character_col character,
    character_col_2 character(255),

    -- varchar
    varchar_col varchar, 
    varchar_col_2 varchar(255), 
    character_varying_col character varying,
    character_varying_col_2 character varying(255),

    bpchar_col bpchar,
    bpchar_col_2 bpchar(10),

    text_col text, 

    -- float
    real_col real, 
    float4_col float4,

    -- double
    double_precision_col double precision, 
    float8_col float8,

    -- decimal
    numeric_col numeric, 
    numeric_col_2 numeric(10, 2), 
    decimal_col decimal, 
    decimal_col_2 decimal(10, 2), 

    -- small int
    smallint_col smallint, 
    int2_col int2,
    smallserial_col smallserial,
    serial2_col smallserial,

    -- int
    integer_col integer,
    int_col int,
    int4_col int,
    serial_col serial,
    serial4_col serial4,

    -- bigint
    bigint_col bigint, 
    int8_col int8, 
    bigserial_col bigserial,
    serial8_col serial8,

    -- bit
    bit_col bit,
    bit_col_2 bit(10),
    bit_varying_col bit varying,
    bit_varying_col_2 bit varying(10),
    varbit_col varbit,
    varbit_col_2 varbit(10),

    -- time
    time_col time, 
    time_col_2 time(6),
    time_col_3 time without time zone,
    time_col_4 time(6) without time zone,

    -- timez
    timez_col timetz,
    timez_col_2 timetz(6),
    timez_col_3 time with time zone,
    timez_col_4 time(6) with time zone,

    -- timestamp
    timestamp_col timestamp, 
    timestamp_col_2 timestamp(6),
    timestamp_col_3 timestamp without time zone,
    timestamp_col_4 timestamp(6) without time zone,

    -- timestampz
    timestampz_col timestamptz,
    timestampz_col_2 timestamptz(6),
    timestampz_col_3 timestamp with time zone,
    timestampz_col_4 timestamp(6) with time zone,

    date_col date, 
    
    bytea_col bytea, 

    -- bool
    boolean_col boolean, 
    bool_col bool,

    -- json
    json_col json, 
    jsonb_col jsonb, 

    -- interval
    interval_col interval, 
    interval_col_2 interval(3), 

    -- array
    array_float4_col float4[],
    array_float8_col float8[],

    array_int2_col int2[],
    array_int4_col int4[],
    array_int8_col bigint[],
    array_int8_col_2 int8[],

    array_text_col text[],

    array_boolean_col boolean[],
    array_boolean_col_2 bool[],

    array_date_col date[],

    array_timestamp_col timestamp[],
    array_timestamp_col_2 timestamp(6)[],
    array_timestamptz_col timestamptz[],
    array_timestamptz_col_2 timestamptz(6)[],

    -- others
    box_col box, 
    cidr_col cidr,
    circle_col circle,
    inet_col inet,

    line_col line,
    lseg_col lseg, 
    macaddr_col macaddr,
    macaddr8_col macaddr8,
    money_col money,

    path_col path, 
    pg_lsn_col pg_lsn,
    pg_snapshot_col pg_snapshot,
    polygon_col polygon, 
    point_col point, 

    tsquery_col tsquery,
    tsvector_col tsvector,
    txid_snapshot_col txid_snapshot,

    uuid_col uuid, 
    xml_col xml
);
```

- The generated sql to be executed in ClickHouse when migrate structures by ape_dts:
```
CREATE TABLE test_db_1.full_column_type
(
    `id` Int32,
    `char_col` Nullable(String),
    `char_col_2` Nullable(String),
    `character_col` Nullable(String),
    `character_col_2` Nullable(String),
    `varchar_col` Nullable(String),
    `varchar_col_2` Nullable(String),
    `character_varying_col` Nullable(String),
    `character_varying_col_2` Nullable(String),
    `bpchar_col` Nullable(String),
    `bpchar_col_2` Nullable(String),
    `text_col` Nullable(String),
    `real_col` Nullable(Float32),
    `float4_col` Nullable(Float32),
    `double_precision_col` Nullable(Float64),
    `float8_col` Nullable(Float64),
    `numeric_col` Nullable(Decimal(38, 9)),
    `numeric_col_2` Nullable(Decimal(38, 9)),
    `decimal_col` Nullable(Decimal(38, 9)),
    `decimal_col_2` Nullable(Decimal(38, 9)),
    `smallint_col` Nullable(Int16),
    `int2_col` Nullable(Int16),
    `smallserial_col` Int16,
    `serial2_col` Int16,
    `integer_col` Nullable(Int32),
    `int_col` Nullable(Int32),
    `int4_col` Nullable(Int32),
    `serial_col` Int32,
    `serial4_col` Int32,
    `bigint_col` Nullable(Int64),
    `int8_col` Nullable(Int64),
    `bigserial_col` Int64,
    `serial8_col` Int64,
    `bit_col` Nullable(String),
    `bit_col_2` Nullable(String),
    `bit_varying_col` Nullable(String),
    `bit_varying_col_2` Nullable(String),
    `varbit_col` Nullable(String),
    `varbit_col_2` Nullable(String),
    `time_col` Nullable(String),
    `time_col_2` Nullable(String),
    `time_col_3` Nullable(String),
    `time_col_4` Nullable(String),
    `timez_col` Nullable(String),
    `timez_col_2` Nullable(String),
    `timez_col_3` Nullable(String),
    `timez_col_4` Nullable(String),
    `timestamp_col` Nullable(DateTime64(6)),
    `timestamp_col_2` Nullable(DateTime64(6)),
    `timestamp_col_3` Nullable(DateTime64(6)),
    `timestamp_col_4` Nullable(DateTime64(6)),
    `timestampz_col` Nullable(DateTime64(6)),
    `timestampz_col_2` Nullable(DateTime64(6)),
    `timestampz_col_3` Nullable(DateTime64(6)),
    `timestampz_col_4` Nullable(DateTime64(6)),
    `date_col` Nullable(Date32),
    `bytea_col` Nullable(String),
    `boolean_col` Nullable(Bool),
    `bool_col` Nullable(Bool),
    `json_col` Nullable(String),
    `jsonb_col` Nullable(String),
    `interval_col` Nullable(String),
    `interval_col_2` Nullable(String),
    `array_float4_col` Nullable(String),
    `array_float8_col` Nullable(String),
    `array_int2_col` Nullable(String),
    `array_int4_col` Nullable(String),
    `array_int8_col` Nullable(String),
    `array_int8_col_2` Nullable(String),
    `array_text_col` Nullable(String),
    `array_boolean_col` Nullable(String),
    `array_boolean_col_2` Nullable(String),
    `array_date_col` Nullable(String),
    `array_timestamp_col` Nullable(String),
    `array_timestamp_col_2` Nullable(String),
    `array_timestamptz_col` Nullable(String),
    `array_timestamptz_col_2` Nullable(String),
    `box_col` Nullable(String),
    `cidr_col` Nullable(String),
    `circle_col` Nullable(String),
    `inet_col` Nullable(String),
    `line_col` Nullable(String),
    `lseg_col` Nullable(String),
    `macaddr_col` Nullable(String),
    `macaddr8_col` Nullable(String),
    `money_col` Nullable(String),
    `path_col` Nullable(String),
    `pg_lsn_col` Nullable(String),
    `pg_snapshot_col` Nullable(String),
    `polygon_col` Nullable(String),
    `point_col` Nullable(String),
    `tsquery_col` Nullable(String),
    `tsvector_col` Nullable(String),
    `txid_snapshot_col` Nullable(String),
    `uuid_col` Nullable(UUID),
    `xml_col` Nullable(String),
    `_ape_dts_is_deleted` Int8,
    `_ape_dts_timestamp` Int64
)
ENGINE = ReplacingMergeTree(_ape_dts_timestamp)
PRIMARY KEY id
ORDER BY id
SETTINGS index_granularity = 8192
```

# DDL during CDC is NOT supported yet
Currently, DDL events are ignored, we may support this in future.
---

### Document: docs/en/tutorial/pg_to_doris.md

# Migrate data from Postgres to Doris

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/pg_to_doris.md) and [common configs](/docs/en/config.md) for more details.

# Prepare Postgres instance
Refer to [pg to pg](./pg_to_pg.md)

# Prepare Doris instance

```
docker run -itd --name some-doris \
-p 9030:9030 \
-p 8030:8030 \
-p 8040:8040 \
"$DORIS_IMAGE"
```

# Migrate structures
## Prepare source data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

CREATE SCHEMA test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
extract_type=struct
db_type=pg
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
url=mysql://root:@127.0.0.1:9030
sink_type=struct
db_type=doris

[filter]
do_dbs=test_db

[parallelizer]
parallel_type=serial

[pipeline]
buffer_size=100
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="Doris > "

SHOW CREATE TABLE test_db.tb_1;
```

```
CREATE TABLE `tb_1` (
  `id` INT NOT NULL,
  `value` INT NULL
) ENGINE=OLAP
UNIQUE KEY(`id`)
COMMENT 'OLAP'
DISTRIBUTED BY HASH(`id`) BUCKETS 10
PROPERTIES (
"replication_allocation" = "tag.location.default: 1",
"min_load_replica_num" = "-1",
"is_being_synced" = "false",
"storage_medium" = "hdd",
"storage_format" = "V2",
"enable_unique_key_merge_on_write" = "true",
"light_schema_change" = "true",
"disable_auto_compaction" = "false",
"enable_single_replica_compaction" = "false",
"group_commit_interval_ms" = "10000",
"group_commit_data_bytes" = "134217728"
);
```

# Migrate snapshot data
## Prepare source data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=snapshot
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
db_type=doris
sink_type=write
url=mysql://root:@127.0.0.1:9030
stream_load_url=mysql://root:@127.0.0.1:8040
batch_size=5000

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="Doris > "

SELECT * FROM test_db.tb_1;
```

```
+------+-------+
| id   | value |
+------+-------+
|    1 |     1 |
|    2 |     2 |
|    3 |     3 |
|    4 |     4 |
+------+-------+
```

# Cdc task

## Drop replication slot if exists
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

SELECT pg_drop_replication_slot('ape_test') FROM pg_replication_slots WHERE slot_name = 'ape_test';
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=cdc
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
slot_name=ape_test

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=doris
sink_type=write
url=mysql://root:@127.0.0.1:9030
stream_load_url=mysql://root:@127.0.0.1:8040
batch_size=5000

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=2000000 WHERE id=2;
INSERT INTO test_db.tb_1 VALUES(5,5);
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="Doris > "

SELECT * FROM test_db.tb_1;
```

```
+------+---------+
| id   | value   |
+------+---------+
|    2 | 2000000 |
|    3 |       3 |
|    4 |       4 |
|    5 |       5 |
+------+---------+
```

# How it works

Refer to [mysql to doris](/docs/en/tutorial/mysql_to_doris.md)

# Data type mapping
- Create a table in Postgres

```
CREATE SCHEMA test_db;

CREATE TABLE test_db.full_column_type (
    id serial PRIMARY KEY, 

    -- char
    char_col char,
    char_col_2 char(255),
    character_col character,
    character_col_2 character(255),

    -- varchar
    varchar_col varchar, 
    varchar_col_2 varchar(255), 
    character_varying_col character varying,
    character_varying_col_2 character varying(255),

    bpchar_col bpchar,
    bpchar_col_2 bpchar(10),

    text_col text, 

    -- float
    real_col real, 
    float4_col float4,

    -- double
    double_precision_col double precision, 
    float8_col float8,

    -- decimal
    numeric_col numeric, 
    numeric_col_2 numeric(10, 2), 
    decimal_col decimal, 
    decimal_col_2 decimal(10, 2), 

    -- small int
    smallint_col smallint, 
    int2_col int2,
    smallserial_col smallserial,
    serial2_col smallserial,

    -- int
    integer_col integer,
    int_col int,
    int4_col int,
    serial_col serial,
    serial4_col serial4,

    -- bigint
    bigint_col bigint, 
    int8_col int8, 
    bigserial_col bigserial,
    serial8_col serial8,

    -- bit
    bit_col bit,
    bit_col_2 bit(10),
    bit_varying_col bit varying,
    bit_varying_col_2 bit varying(10),
    varbit_col varbit,
    varbit_col_2 varbit(10),

    -- time
    time_col time, 
    time_col_2 time(6),
    time_col_3 time without time zone,
    time_col_4 time(6) without time zone,

    -- timez
    timez_col timetz,
    timez_col_2 timetz(6),
    timez_col_3 time with time zone,
    timez_col_4 time(6) with time zone,

    -- timestamp
    timestamp_col timestamp, 
    timestamp_col_2 timestamp(6),
    timestamp_col_3 timestamp without time zone,
    timestamp_col_4 timestamp(6) without time zone,

    -- timestampz
    timestampz_col timestamptz,
    timestampz_col_2 timestamptz(6),
    timestampz_col_3 timestamp with time zone,
    timestampz_col_4 timestamp(6) with time zone,

    date_col date, 
    
    bytea_col bytea, 

    -- bool
    boolean_col boolean, 
    bool_col bool,

    -- json
    json_col json, 
    jsonb_col jsonb, 

    -- interval
    interval_col interval, 
    interval_col_2 interval(3), 

    -- array
    array_float4_col float4[],
    array_float8_col float8[],

    array_int2_col int2[],
    array_int4_col int4[],
    array_int8_col bigint[],
    array_int8_col_2 int8[],

    array_text_col text[],

    array_boolean_col boolean[],
    array_boolean_col_2 bool[],

    array_date_col date[],

    array_timestamp_col timestamp[],
    array_timestamp_col_2 timestamp(6)[],
    array_timestamptz_col timestamptz[],
    array_timestamptz_col_2 timestamptz(6)[],

    -- others
    box_col box, 
    cidr_col cidr,
    circle_col circle,
    inet_col inet,

    line_col line,
    lseg_col lseg, 
    macaddr_col macaddr,
    macaddr8_col macaddr8,
    money_col money,

    path_col path, 
    pg_lsn_col pg_lsn,
    pg_snapshot_col pg_snapshot,
    polygon_col polygon, 
    point_col point, 

    tsquery_col tsquery,
    tsvector_col tsvector,
    txid_snapshot_col txid_snapshot,

    uuid_col uuid, 
    xml_col xml
);
```

- The generated sql to be executed in Doris when migrate structures by ape_dts:
```
CREATE TABLE IF NOT EXISTS `test_db`.`full_column_type` (
  `id` INT NOT NULL, 
  `char_col` STRING, 
  `char_col_2` STRING, 
  `character_col` STRING, 
  `character_col_2` STRING, 
  `varchar_col` STRING, 
  `varchar_col_2` STRING, 
  `character_varying_col` STRING, 
  `character_varying_col_2` STRING, 
  `bpchar_col` STRING, 
  `bpchar_col_2` STRING, 
  `text_col` STRING, 
  `real_col` FLOAT, 
  `float4_col` FLOAT, 
  `double_precision_col` DOUBLE, 
  `float8_col` DOUBLE, 
  `numeric_col` DECIMAL(38, 9), 
  `numeric_col_2` DECIMAL(38, 9), 
  `decimal_col` DECIMAL(38, 9), 
  `decimal_col_2` DECIMAL(38, 9), 
  `smallint_col` SMALLINT, 
  `int2_col` SMALLINT, 
  `smallserial_col` SMALLINT, 
  `serial2_col` SMALLINT, 
  `integer_col` INT, 
  `int_col` INT, 
  `int4_col` INT, 
  `serial_col` INT, 
  `serial4_col` INT, 
  `bigint_col` BIGINT, 
  `int8_col` BIGINT, 
  `bigserial_col` BIGINT, 
  `serial8_col` BIGINT, 
  `bit_col` STRING, 
  `bit_col_2` STRING, 
  `bit_varying_col` STRING, 
  `bit_varying_col_2` STRING, 
  `varbit_col` STRING, 
  `varbit_col_2` STRING, 
  `time_col` VARCHAR(255), 
  `time_col_2` VARCHAR(255), 
  `time_col_3` VARCHAR(255), 
  `time_col_4` VARCHAR(255), 
  `timez_col` VARCHAR(255), 
  `timez_col_2` VARCHAR(255), 
  `timez_col_3` VARCHAR(255), 
  `timez_col_4` VARCHAR(255), 
  `timestamp_col` DATETIME(6), 
  `timestamp_col_2` DATETIME(6), 
  `timestamp_col_3` DATETIME(6), 
  `timestamp_col_4` DATETIME(6), 
  `timestampz_col` DATETIME(6), 
  `timestampz_col_2` DATETIME(6), 
  `timestampz_col_3` DATETIME(6), 
  `timestampz_col_4` DATETIME(6), 
  `date_col` DATE, 
  `bytea_col` STRING, 
  `boolean_col` BOOLEAN, 
  `bool_col` BOOLEAN, 
  `json_col` JSON, 
  `jsonb_col` JSON, 
  `interval_col` VARCHAR(255), 
  `interval_col_2` VARCHAR(255), 
  `array_float4_col` STRING, 
  `array_float8_col` STRING, 
  `array_int2_col` STRING, 
  `array_int4_col` STRING, 
  `array_int8_col` STRING, 
  `array_int8_col_2` STRING, 
  `array_text_col` STRING, 
  `array_boolean_col` STRING, 
  `array_boolean_col_2` STRING, 
  `array_date_col` STRING, 
  `array_timestamp_col` STRING, 
  `array_timestamp_col_2` STRING, 
  `array_timestamptz_col` STRING, 
  `array_timestamptz_col_2` STRING, 
  `box_col` STRING, 
  `cidr_col` STRING, 
  `circle_col` STRING, 
  `inet_col` STRING, 
  `line_col` STRING, 
  `lseg_col` STRING, 
  `macaddr_col` STRING, 
  `macaddr8_col` STRING, 
  `money_col` STRING, 
  `path_col` STRING, 
  `pg_lsn_col` STRING, 
  `pg_snapshot_col` STRING, 
  `polygon_col` STRING, 
  `point_col` STRING, 
  `tsquery_col` STRING, 
  `tsvector_col` STRING, 
  `txid_snapshot_col` STRING, 
  `uuid_col` STRING, 
  `xml_col` STRING
) UNIQUE KEY (`id`) DISTRIBUTED BY HASH(`id`) PROPERTIES ("replication_num" = "1")
```

# Supported versions

Refer to [mysql to doris](/docs/en/tutorial/mysql_to_doris.md)

# DDL during CDC is NOT supported yet
Currently, DDL events are ignored, we may support this in future.
---

### Document: docs/en/tutorial/pg_to_http_server_consumer.md

# Start as HTTP server and extract Postgres data

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/rdb_to_http_server.md) and [common configs](/docs/en/config.md) for more details.

- Refer to [Start ape_dts as HTTP server to provide data to consumers](/docs/en/consumer/http_consumer.md) for task description.

# Prepare Postgres instances
Refer to [pg to pg](./pg_to_pg.md)

# CDC task

## Drop replication slot if exists
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

SELECT pg_drop_replication_slot('ape_test') FROM pg_replication_slots WHERE slot_name = 'ape_test';
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=cdc
url=postgres://postgres:postgres@host.docker.internal:5433/postgres?options[statement_timeout]=10s
slot_name=ape_test

[sinker]
sink_type=dummy

[parallelizer]
parallel_type=serial
parallel_size=1

[filter]
do_dbs=test_db,test_db_2
do_events=insert,update,delete
do_ddls=*

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
pipeline_type=http_server
http_host=0.0.0.0
http_port=10231
with_field_defs=true
EOL
```

```
docker run --rm \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-p 10231:10231 \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Make changes in Postgres
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

CREATE SCHEMA test_db_2;
CREATE TABLE test_db_2.tb_2(id int, value int, primary key(id));
INSERT INTO test_db_2.tb_2 VALUES(1,1);
UPDATE test_db_2.tb_2 SET value=100000 WHERE id=1;
DELETE FROM test_db_2.tb_2;
```

# Start consumer

[python / golang consumer demo](https://github.com/apecloud/ape_dts_consumer_demo)
---

### Document: docs/en/tutorial/pg_to_kafka_consumer.md

# Send Postgres data to Kafka

Refer to [Send data to Kafka](/docs/en/consumer/kafka_consumer.md) for consumers.

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/rdb_to_kafka.md) and [common configs](/docs/en/config.md) for more details.

# Prepare Postgres instance
Refer to [pg to pg](./pg_to_pg.md)

# Prepare Kafka instance
Refer to [mysql to kafka](./mysql_to_kafka_consumer.md)

# Send Snapshot data to Kafka
## Prepare data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

CREATE SCHEMA test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=snapshot
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
db_type=kafka
sink_type=write
url=127.0.0.1:9093
with_field_defs=true

[filter]
do_dbs=test_db
do_events=insert

[router]
topic_map=*.*:test

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

# Send CDC data to Kafka

## Drop replication slot if exists
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

SELECT pg_drop_replication_slot('ape_test') FROM pg_replication_slots WHERE slot_name = 'ape_test';
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=cdc
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
slot_name=ape_test

[filter]
do_dbs=test_db,test_db_2
do_events=insert,update,delete
do_ddls=*

[router]
topic_map=*.*:test

[sinker]
db_type=kafka
sink_type=write
url=127.0.0.1:9093
with_field_defs=true

[parallelizer]
parallel_type=serial
parallel_size=1

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Make changes in Postgres
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

CREATE SCHEMA test_db_2;
CREATE TABLE test_db_2.tb_2(id int, value int, primary key(id));
INSERT INTO test_db_2.tb_2 VALUES(1,1);
UPDATE test_db_2.tb_2 SET value=100000 WHERE id=1;
DELETE FROM test_db_2.tb_2;
```

# Run Kafka consumer demo

[python / golang consumer demo](https://github.com/apecloud/ape_dts_consumer_demo)
---

### Document: docs/en/tutorial/pg_to_pg.md

# Migrate data from Postgres to Postgres

# Prerequisites

- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/pg_to_pg.md) and [common configs](/docs/en/config.md) for more details.

# Prepare Postgres instances

## Source

```
docker run --name some-postgres-1 \
-p 5433:5432 \
-e POSTGRES_PASSWORD=postgres \
-e TZ=Etc/GMT-8 \
-d "$POSTGRES_IMAGE"
```

- set wal_level to logical

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

ALTER SYSTEM SET wal_level = logical;

-- restart container
docker restart some-postgres-1
```

## Target

```
docker run --name some-postgres-2 \
-p 5434:5432 \
-e POSTGRES_PASSWORD=postgres \
-e TZ=Etc/GMT-7 \
-d "$POSTGRES_IMAGE"
```

# Migrate structures

## Prepare data

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

CREATE SCHEMA test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
```

## Start task

```
rm -rf /tmp/ape_dts
mkdir -p /tmp/ape_dts

cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
extract_type=struct
db_type=pg
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
sink_type=struct
db_type=pg
url=postgres://postgres:postgres@127.0.0.1:5434/postgres?options[statement_timeout]=10s

[filter]
do_dbs=test_db

[parallelizer]
parallel_type=serial

[pipeline]
buffer_size=100
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini
```

## Check results

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5434 -W

SET search_path TO test_db;
\d
```

```
         List of relations
 Schema  | Name | Type  |  Owner
---------+------+-------+----------
 test_db | tb_1 | table | postgres
```

# Migrate snapshot data

## Prepare data

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

## Start task

```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=snapshot
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
db_type=pg
sink_type=write
url=postgres://postgres:postgres@127.0.0.1:5434/postgres?options[statement_timeout]=10s

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini
```

## Check results

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5434 -W

SELECT * FROM test_db.tb_1 ORDER BY id;
```

```
 id | value
----+-------
  1 |     1
  2 |     2
  3 |     3
  4 |     4
```

# Check data

- check the differences between target data and source data

## Prepare data

- change target table records

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5434 -W

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=1 WHERE id=2;
```

## Start task

```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=snapshot
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
db_type=pg
sink_type=check
url=postgres://postgres:postgres@127.0.0.1:5434/postgres?options[statement_timeout]=10s

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=rdb_check
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-v "/tmp/ape_dts/check_data_task_log/:/logs/" \
"$APE_DTS_IMAGE" /task_config.ini
```

## Check results

- cat /tmp/ape_dts/check_data_task_log/check/miss.log

```
{"log_type":"Miss","schema":"test_db","tb":"tb_1","id_col_values":{"id":"1"},"diff_col_values":{}}
```

- cat /tmp/ape_dts/check_data_task_log/check/diff.log

```
{"log_type":"Diff","schema":"test_db","tb":"tb_1","id_col_values":{"id":"2"},"diff_col_values":{"value":{"src":"2","dst":"1"}}}
```

# Revise data

- revise target data based on "check data" task results

## Start task

```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=check_log
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
check_log_dir=./check_data_task_log

[sinker]
db_type=pg
sink_type=write
url=postgres://postgres:postgres@127.0.0.1:5434/postgres?options[statement_timeout]=10s

[filter]
do_events=*

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-v "/tmp/ape_dts/check_data_task_log/check/:/check_data_task_log/" \
"$APE_DTS_IMAGE" /task_config.ini
```

## Check results

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5434 -W

SELECT * FROM test_db.tb_1 ORDER BY id;
```

```
 id | value
----+-------
  1 |     1
  2 |     2
  3 |     3
  4 |     4
```

# Review data

- check if target data revised based on "check data" task results

## Start task

```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=check_log
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
check_log_dir=./check_data_task_log

[sinker]
db_type=pg
sink_type=check
url=postgres://postgres:postgres@127.0.0.1:5434/postgres?options[statement_timeout]=10s

[filter]
do_events=*

[parallelizer]
parallel_type=rdb_check
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
-v "/tmp/ape_dts/check_data_task_log/check/:/check_data_task_log/" \
-v "/tmp/ape_dts/review_data_task_log/:/logs/" \
"$APE_DTS_IMAGE" /task_config.ini
```

## Check results

- /tmp/ape_dts/review_data_task_log/check/miss.log and /tmp/ape_dts/review_data_task_log/check/diff.log should be empty

# CDC task

## Drop replication slot if exists

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

SELECT pg_drop_replication_slot('ape_test') FROM pg_replication_slots WHERE slot_name = 'ape_test';
```

## Start task

- this will create slot if not exists.

```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=cdc
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
slot_name=ape_test

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=pg
sink_type=write
batch_size=200
url=postgres://postgres:postgres@127.0.0.1:5434/postgres?options[statement_timeout]=10s

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini
```

## Change source data

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=2000000 WHERE id=2;
INSERT INTO test_db.tb_1 VALUES(5,5);
```

## Check results

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5434 -W

SELECT * FROM test_db.tb_1 ORDER BY id;
```

```
 id |  value
----+---------
  2 | 2000000
  3 |       3
  4 |       4
  5 |       5
```

# CDC task with ddl capture

## Enable ddl capture in source

- Create a meta table to store ddl info

```
CREATE TABLE public.ape_dts_ddl_command
(
  ddl_text text COLLATE pg_catalog."default",
  id bigserial primary key,
  event text COLLATE pg_catalog."default",
  tag text COLLATE pg_catalog."default",
  username character varying COLLATE pg_catalog."default",
  database character varying COLLATE pg_catalog."default",
  schema character varying COLLATE pg_catalog."default",
  object_type character varying COLLATE pg_catalog."default",
  object_name character varying COLLATE pg_catalog."default",
  client_address character varying COLLATE pg_catalog."default",
  client_port integer,
  event_time timestamp with time zone,
  txid_current character varying(128) COLLATE pg_catalog."default",
  message text COLLATE pg_catalog."default"
);
```

- Create a function to capture ddl and record it into ddl meta table

```
CREATE FUNCTION public.ape_dts_capture_ddl()
  RETURNS event_trigger
  LANGUAGE 'plpgsql'
  COST 100
  VOLATILE NOT LEAKPROOF SECURITY DEFINER
AS $BODY$
  declare ddl_text text;
  declare max_rows int := 10000;
  declare current_rows int;
  declare pg_version_95 int := 90500;
  declare pg_version_10 int := 100000;
  declare current_version int;
  declare object_id varchar;
  declare alter_table varchar;
  declare record_object record;
  declare message text;
  declare pub RECORD;
begin

  select current_query() into ddl_text;

  if TG_TAG = 'CREATE TABLE' then -- ALTER TABLE schema.TABLE REPLICA IDENTITY FULL;
    show server_version_num into current_version;
    if current_version >= pg_version_95 then
      for record_object in (select * from pg_event_trigger_ddl_commands()) loop
        if record_object.command_tag = 'CREATE TABLE' then
          object_id := record_object.object_identity;
        end if;
      end loop;
    else
      select btrim(substring(ddl_text from '[ \t\r\n\v\f]*[c|C][r|R][e|E][a|A][t|T][e|E][ \t\r\n\v\f]*.*[ \t\r\n\v\f]*[t|T][a|A][b|B][l|L][e|E][ \t\r\n\v\f]+(.*)\(.*'),' \t\r\n\v\f') into object_id;
    end if;
    if object_id = '' or object_id is null then
      message := 'CREATE TABLE, but ddl_text=' || ddl_text || ', current_query=' || current_query();
    end if;
    if current_version >= pg_version_10 then
      for pub in (select * from pg_publication where pubname like 'ape_dts_%') loop
        raise notice 'pubname=%',pub.pubname;
        BEGIN
          execute 'alter publication ' || pub.pubname || ' add table ' || object_id;
        EXCEPTION WHEN OTHERS THEN
        END;
      end loop;
    end if;
  end if;

  insert into public.ape_dts_ddl_command(id,event,tag,username,database,schema,object_type,object_name,client_address,client_port,event_time,ddl_text,txid_current,message)
  values (default,TG_EVENT,TG_TAG,current_user,current_database(),current_schema,'','',inet_client_addr(),inet_client_port(),current_timestamp,ddl_text,cast(TXID_CURRENT() as varchar(16)),message);

  select count(id) into current_rows from public.ape_dts_ddl_command;
  if current_rows > max_rows then
    delete from public.ape_dts_ddl_command where id in (select min(id) from public.ape_dts_ddl_command);
  end if;
end
$BODY$;
```

- Alter the function owner to your account

```
ALTER FUNCTION public.ape_dts_capture_ddl() OWNER TO postgres;
```

- Create an event trigger on ddl_command_end and execute the capture function

```
CREATE EVENT TRIGGER ape_dts_intercept_ddl ON ddl_command_end
EXECUTE PROCEDURE public.ape_dts_capture_ddl();
```

## Start task

```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=cdc
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
slot_name=ape_test
ddl_meta_tb=public.ape_dts_ddl_command

[filter]
do_dbs=test_db
do_events=insert,update,delete
do_ddls=create_schema,drop_schema,alter_schema,create_table,alter_table,drop_table,create_index,drop_index,truncate_table,rename_table

[sinker]
db_type=pg
sink_type=write
batch_size=200
url=postgres://postgres:postgres@127.0.0.1:5434/postgres?options[statement_timeout]=10s

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini
```

## Do ddls in source

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

CREATE TABLE test_db.tb_2(id int, value int, primary key(id));
INSERT INTO test_db.tb_2 VALUES(1,1);
```

## Check results

```
psql -h 127.0.0.1 -U postgres -d postgres -p 5434 -W

SELECT * FROM test_db.tb_2 ORDER BY id;
```

```
 id | value
----+-------
  1 |     1
```

---

### Document: docs/en/tutorial/pg_to_starrocks.md

# Migrate data from Postgres to StarRocks

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/pg_to_starrocks.md) and [common configs](/docs/en/config.md) for more details.

# Prepare Postgres instance
Refer to [pg to pg](./pg_to_pg.md)

# Prepare StarRocks instance
- tested versions: 2.5.4 to 3.2.11

```
docker run -itd --name some-starrocks \
-p 9030:9030 \
-p 8030:8030 \
-p 8040:8040 \
"$STARROCKS_IMAGE"
```

# Migrate structures
## Prepare source data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

CREATE SCHEMA test_db;
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
extract_type=struct
db_type=pg
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
url=mysql://root:@127.0.0.1:9030
sink_type=struct
db_type=starrocks

[filter]
do_dbs=test_db

[parallelizer]
parallel_type=serial

[pipeline]
buffer_size=100
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="StarRocks > "

SHOW CREATE TABLE test_db.tb_1;
```

```
CREATE TABLE `tb_1` (
  `id` int(11) NOT NULL COMMENT "",
  `value` int(11) NULL COMMENT "",
  `_ape_dts_is_deleted` boolean NULL COMMENT "",
  `_ape_dts_timestamp` bigint(20) NULL COMMENT ""
) ENGINE=OLAP 
PRIMARY KEY(`id`)
DISTRIBUTED BY HASH(`id`)
PROPERTIES (
"replication_num" = "1",
"in_memory" = "false",
"enable_persistent_index" = "true",
"replicated_storage" = "true",
"compression" = "LZ4"
);
```

# Migrate snapshot data
## Prepare source data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

INSERT INTO test_db.tb_1 VALUES(1,1),(2,2),(3,3),(4,4);
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=snapshot
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s

[sinker]
db_type=starrocks
sink_type=write
url=mysql://root:@127.0.0.1:9030
stream_load_url=mysql://root:@127.0.0.1:8040
batch_size=5000

[filter]
do_dbs=test_db
do_events=insert

[parallelizer]
parallel_type=snapshot
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="StarRocks > "

SELECT * FROM test_db.tb_1;
```

```
+------+-------+---------------------+--------------------+
| id   | value | _ape_dts_is_deleted | _ape_dts_timestamp |
+------+-------+---------------------+--------------------+
|    1 |     1 |                NULL |    1731665154965   |
|    2 |     2 |                NULL |    1731665159858   |
|    3 |     3 |                NULL |    1731665159880   |
|    4 |     4 |                NULL |    1731665159880   |
+------+-------+---------------------+--------------------+
```

# Cdc task with hard delete (NOT recommended)

## Drop replication slot if exists
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

SELECT pg_drop_replication_slot('ape_test') FROM pg_replication_slots WHERE slot_name = 'ape_test';
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=cdc
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
slot_name=ape_test

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=starrocks
sink_type=write
url=mysql://root:@127.0.0.1:9030
stream_load_url=mysql://root:@127.0.0.1:8040
hard_delete=true
batch_size=5000

[parallelizer]
parallel_type=rdb_merge
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

DELETE FROM test_db.tb_1 WHERE id=1;
UPDATE test_db.tb_1 SET value=2000000 WHERE id=2;
INSERT INTO test_db.tb_1 VALUES(5,5);
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="StarRocks > "

SELECT * FROM test_db.tb_1;
```

```
+------+---------+---------------------+--------------------+
| id   | value   | _ape_dts_is_deleted | _ape_dts_timestamp |
+------+---------+---------------------+--------------------+
|    2 | 2000000 |                NULL |    1731665679461   |
|    3 |       3 |                NULL |    1731665609225   |
|    4 |       4 |                NULL |    1731665609236   |
|    5 |       5 |                NULL |    1731665679572   |
+------+---------+---------------------+--------------------+
```

# Cdc task with soft delete (recommended)
## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=pg
extract_type=cdc
url=postgres://postgres:postgres@127.0.0.1:5433/postgres?options[statement_timeout]=10s
slot_name=ape_test

[filter]
do_dbs=test_db
do_events=insert,update,delete

[sinker]
db_type=starrocks
sink_type=write
url=mysql://root:@127.0.0.1:9030
stream_load_url=mysql://root:@127.0.0.1:8040
batch_size=5000

[parallelizer]
parallel_type=table
parallel_size=8

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
psql -h 127.0.0.1 -U postgres -d postgres -p 5433 -W

DELETE FROM test_db.tb_1 WHERE id=3;
```

## Check results
```
mysql -P 9030 -h 127.0.0.1 -u root --prompt="StarRocks > "

SELECT * FROM test_db.tb_1;
```

```
+------+---------+---------------------+--------------------+
| id   | value   | _ape_dts_is_deleted | _ape_dts_timestamp |
+------+---------+---------------------+--------------------+
|    2 | 2000000 |                NULL |    1731665679461   |
|    3 |       3 |                   1 |    1731665747381   |
|    4 |       4 |                NULL |    1731665609236   |
|    5 |       5 |                NULL |    1731665679572   |
+------+---------+---------------------+--------------------+
```

# How it works

Refer to [mysql to starrocks](/docs/en/tutorial/mysql_to_starrocks.md)

# Data type mapping
- Create a table in Postgres

```
CREATE SCHEMA test_db;

CREATE TABLE test_db.full_column_type (
    id serial PRIMARY KEY, 

    -- char
    char_col char,
    char_col_2 char(255),
    character_col character,
    character_col_2 character(255),

    -- varchar
    varchar_col varchar, 
    varchar_col_2 varchar(255), 
    character_varying_col character varying,
    character_varying_col_2 character varying(255),

    bpchar_col bpchar,
    bpchar_col_2 bpchar(10),

    text_col text, 

    -- float
    real_col real, 
    float4_col float4,

    -- double
    double_precision_col double precision, 
    float8_col float8,

    -- decimal
    numeric_col numeric, 
    numeric_col_2 numeric(10, 2), 
    decimal_col decimal, 
    decimal_col_2 decimal(10, 2), 

    -- small int
    smallint_col smallint, 
    int2_col int2,
    smallserial_col smallserial,
    serial2_col smallserial,

    -- int
    integer_col integer,
    int_col int,
    int4_col int,
    serial_col serial,
    serial4_col serial4,

    -- bigint
    bigint_col bigint, 
    int8_col int8, 
    bigserial_col bigserial,
    serial8_col serial8,

    -- bit
    bit_col bit,
    bit_col_2 bit(10),
    bit_varying_col bit varying,
    bit_varying_col_2 bit varying(10),
    varbit_col varbit,
    varbit_col_2 varbit(10),

    -- time
    time_col time, 
    time_col_2 time(6),
    time_col_3 time without time zone,
    time_col_4 time(6) without time zone,

    -- timez
    timez_col timetz,
    timez_col_2 timetz(6),
    timez_col_3 time with time zone,
    timez_col_4 time(6) with time zone,

    -- timestamp
    timestamp_col timestamp, 
    timestamp_col_2 timestamp(6),
    timestamp_col_3 timestamp without time zone,
    timestamp_col_4 timestamp(6) without time zone,

    -- timestampz
    timestampz_col timestamptz,
    timestampz_col_2 timestamptz(6),
    timestampz_col_3 timestamp with time zone,
    timestampz_col_4 timestamp(6) with time zone,

    date_col date, 
    
    bytea_col bytea, 

    -- bool
    boolean_col boolean, 
    bool_col bool,

    -- json
    json_col json, 
    jsonb_col jsonb, 

    -- interval
    interval_col interval, 
    interval_col_2 interval(3), 

    -- array
    array_float4_col float4[],
    array_float8_col float8[],

    array_int2_col int2[],
    array_int4_col int4[],
    array_int8_col bigint[],
    array_int8_col_2 int8[],

    array_text_col text[],

    array_boolean_col boolean[],
    array_boolean_col_2 bool[],

    array_date_col date[],

    array_timestamp_col timestamp[],
    array_timestamp_col_2 timestamp(6)[],
    array_timestamptz_col timestamptz[],
    array_timestamptz_col_2 timestamptz(6)[],

    -- others
    box_col box, 
    cidr_col cidr,
    circle_col circle,
    inet_col inet,

    line_col line,
    lseg_col lseg, 
    macaddr_col macaddr,
    macaddr8_col macaddr8,
    money_col money,

    path_col path, 
    pg_lsn_col pg_lsn,
    pg_snapshot_col pg_snapshot,
    polygon_col polygon, 
    point_col point, 

    tsquery_col tsquery,
    tsvector_col tsvector,
    txid_snapshot_col txid_snapshot,

    uuid_col uuid, 
    xml_col xml
);
```

- The generated sql to be executed in StarRocks when migrate structures by ape_dts:
```
CREATE TABLE IF NOT EXISTS `test_db`.`full_column_type` (
    `id` INT NOT NULL,
    `char_col` STRING,
    `char_col_2` STRING,
    `character_col` STRING,
    `character_col_2` STRING,
    `varchar_col` STRING,
    `varchar_col_2` STRING,
    `character_varying_col` STRING,
    `character_varying_col_2` STRING,
    `bpchar_col` STRING,
    `bpchar_col_2` STRING,
    `text_col` STRING,
    `real_col` FLOAT,
    `float4_col` FLOAT,
    `double_precision_col` DOUBLE,
    `float8_col` DOUBLE,
    `numeric_col` DECIMAL(38, 9),
    `numeric_col_2` DECIMAL(38, 9),
    `decimal_col` DECIMAL(38, 9),
    `decimal_col_2` DECIMAL(38, 9),
    `smallint_col` SMALLINT,
    `int2_col` SMALLINT,
    `smallserial_col` SMALLINT NOT NULL,
    `serial2_col` SMALLINT NOT NULL,
    `integer_col` INT,
    `int_col` INT,
    `int4_col` INT,
    `serial_col` INT NOT NULL,
    `serial4_col` INT NOT NULL,
    `bigint_col` BIGINT,
    `int8_col` BIGINT,
    `bigserial_col` BIGINT NOT NULL,
    `serial8_col` BIGINT NOT NULL,
    `bit_col` STRING,
    `bit_col_2` STRING,
    `bit_varying_col` STRING,
    `bit_varying_col_2` STRING,
    `varbit_col` STRING,
    `varbit_col_2` STRING,
    `time_col` VARCHAR(255),
    `time_col_2` VARCHAR(255),
    `time_col_3` VARCHAR(255),
    `time_col_4` VARCHAR(255),
    `timez_col` VARCHAR(255),
    `timez_col_2` VARCHAR(255),
    `timez_col_3` VARCHAR(255),
    `timez_col_4` VARCHAR(255),
    `timestamp_col` DATETIME,
    `timestamp_col_2` DATETIME,
    `timestamp_col_3` DATETIME,
    `timestamp_col_4` DATETIME,
    `timestampz_col` DATETIME,
    `timestampz_col_2` DATETIME,
    `timestampz_col_3` DATETIME,
    `timestampz_col_4` DATETIME,
    `date_col` DATE,
    `bytea_col` VARBINARY,
    `boolean_col` BOOLEAN,
    `bool_col` BOOLEAN,
    `json_col` JSON,
    `jsonb_col` JSON,
    `interval_col` VARCHAR(255),
    `interval_col_2` VARCHAR(255),
    `array_float4_col` STRING,
    `array_float8_col` STRING,
    `array_int2_col` STRING,
    `array_int4_col` STRING,
    `array_int8_col` STRING,
    `array_int8_col_2` STRING,
    `array_text_col` STRING,
    `array_boolean_col` STRING,
    `array_boolean_col_2` STRING,
    `array_date_col` STRING,
    `array_timestamp_col` STRING,
    `array_timestamp_col_2` STRING,
    `array_timestamptz_col` STRING,
    `array_timestamptz_col_2` STRING,
    `box_col` STRING,
    `cidr_col` STRING,
    `circle_col` STRING,
    `inet_col` STRING,
    `line_col` STRING,
    `lseg_col` STRING,
    `macaddr_col` STRING,
    `macaddr8_col` STRING,
    `money_col` STRING,
    `path_col` STRING,
    `pg_lsn_col` STRING,
    `pg_snapshot_col` STRING,
    `polygon_col` STRING,
    `point_col` STRING,
    `tsquery_col` STRING,
    `tsvector_col` STRING,
    `txid_snapshot_col` STRING,
    `uuid_col` STRING,
    `xml_col` STRING,
    `_ape_dts_is_deleted` BOOLEAN,
    `_ape_dts_timestamp` BIGINT
) PRIMARY KEY (`id`) DISTRIBUTED BY HASH(`id`);
```

# Soft delete or Hard delete 

Refer to [mysql to starrocks](/docs/en/tutorial/mysql_to_starrocks.md)

# Supported versions

Refer to [mysql to starrocks](/docs/en/tutorial/mysql_to_starrocks.md)

# DDL during CDC is NOT supported yet
Currently, DDL events are ignored, we may support this in future.
---

### Document: docs/en/tutorial/prerequisites.md

# Prerequisites
- Docker

- Set images
```
export APE_DTS_IMAGE="docker.io/apecloud/ape-dts:2.0.22"
export MYSQL_IMAGE="mysql:5.7.40"
export POSTGRES_IMAGE="postgis/postgis:15-3.4"
export REDIS_IMAGE="redis:7.0"
export MONGO_IMAGE="mongo:6.0"
export ZOOKEEPER_IMAGE="docker.io/bitnami/zookeeper:3.8"
export KAFKA_IMAGE="docker.io/bitnami/kafka:3.3"
export STARROCKS_IMAGE="starrocks/allin1-ubuntu:3.2.11"
export DORIS_IMAGE="apache/doris:doris-all-in-one-2.1.0"
export CLICKHOUSE_IMAGE="clickhouse/clickhouse-server:24.10"
```
---

### Document: docs/en/tutorial/redis_to_redis.md

# Migrate data from Redis to Redis

# Prerequisites
- [prerequisites](./prerequisites.md)

- This article is for quick start, refer to [templates](/docs/templates/redis_to_redis.md) and [common configs](/docs/en/config.md) for more details.

# Prepare Redis instances

## Source

```
docker run --name src-redis-7-0 \
    -p 6380:6379 \
    -d "$REDIS_IMAGE" redis-server \
    --requirepass 123456 \
    --save 60 1 \
    --loglevel warning
```

## Target

```
docker run --name dst-redis-7-0 \
    -p 6390:6379 \
    -d "$REDIS_IMAGE" redis-server \
    --requirepass 123456 \
    --save 60 1 \
    --loglevel warning
```

# Migrate snapshot data
## Prepare data
```
telnet 127.0.0.1 6380
auth 123456

SELECT 0
SET key_1 val_1

SELECT 1
SET key_2 val_2
```

## Start task
```
rm -rf /tmp/ape_dts
mkdir -p /tmp/ape_dts

cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=redis
extract_type=snapshot
repl_port=10008
url=redis://:123456@127.0.0.1:6380

[filter]
do_dbs=*

[sinker]
db_type=redis
sink_type=write
url=redis://:123456@127.0.0.1:6390

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1

[parallelizer]
parallel_type=redis
parallel_size=8
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Check results
```
telnet 127.0.0.1 6390
auth 123456

SELECT 0
+OK
GET key_1
$5
val_1

SELECT 1
+OK
GET key_2
$5
val_2
```

# Snapshot + Cdc task
- Currently we do not support synchronizing only cdc data, the cdc task will first migrate the snapshot data and then synchronize the cdc data.

## Clear target data
```
telnet 127.0.0.1 6390
auth 123456

flushall
```

## Start task
```
cat <<EOL > /tmp/ape_dts/task_config.ini
[extractor]
db_type=redis
extract_type=snapshot_and_cdc
repl_id=
now_db_id=0
repl_port=10008
repl_offset=0
url=redis://:123456@127.0.0.1:6380

[filter]
do_dbs=*
ignore_cmds=flushall,flushdb

[sinker]
db_type=redis
sink_type=write
method=restore
url=redis://:123456@127.0.0.1:6390

[pipeline]
buffer_size=16000
checkpoint_interval_secs=1

[parallelizer]
parallel_type=redis
parallel_size=8
EOL
```

```
docker run --rm --network host \
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \
"$APE_DTS_IMAGE" /task_config.ini 
```

## Change source data
```
telnet 127.0.0.1 6380
auth 123456

SELECT 0
SET key_3 val_3

SELECT 1
SET key_4 val_4
```

## Check results
```
telnet 127.0.0.1 6390
auth 123456

SELECT 0
+OK
GET key_1
$val_1
GET key_3
$5
val_3

SELECT 1
+OK
GET key_2
$5
val_2
GET key_4
$5
val_4
```
---

### Document: docs/en/tutorial/snapshot_and_cdc_without_data_loss.md

# Execute snapshot and CDC tasks serially without data loss

In most data migration/import scenarios, you may want to migrate snapshot first, then subscribe to source cdc data and synchronize it to target.

This article tells you what to do before starting a snapshot task, and how to configure cdc task_config.ini to avoid data loss.

# Source: MySQL

Refer to [mysql -> mysql](./mysql_to_mysql.md) or [mysql -> kafka](./mysql_to_kafka_consumer.md) to generate snapshot_task_config.ini & cdc_task_config.ini.

- Get binlog info of source MySQL before starting snapshot task

```
mysql -h127.0.0.1 -uroot -p123456 -P3307
show master status;

+------------------+----------+--------------+------------------+-------------------------------------------+
| File             | Position | Binlog_Do_DB | Binlog_Ignore_DB | Executed_Gtid_Set                         |
+------------------+----------+--------------+------------------+-------------------------------------------+
| mysql-bin.000003 |     3009 |              |                  | 9663a096-8adc-11ef-b617-0242ac110002:1-17 |
+------------------+----------+--------------+------------------+-------------------------------------------+
```

- Update cdc_task_config.ini
```
[extractor]
binlog_position=3009
binlog_filename=mysql-bin.000003
```

- Start snapshot task
- Start cdc task once snapshot task finished, all changes made during snapshot task will be synchronized to target.

# Source: Postgres
Refer to [pg -> pg](./pg_to_pg.md) or [pg -> kafka](./pg_to_kafka_consumer.md) to generate snapshot_task_config.ini & cdc_task_config.ini.

- Check if replication slot exists in source Postgres
```
SELECT * FROM pg_catalog.pg_replication_slots WHERE slot_name = 'ape_test';
```

- Drop it if not used by others, or use another slot_name
```
SELECT pg_drop_replication_slot('ape_test') FROM pg_replication_slots WHERE slot_name = 'ape_test';
```

- Create slot and get starting lsn
```
SELECT * FROM pg_create_logical_replication_slot('ape_test', 'pgoutput');
```
```
 slot_name |    lsn    
-----------+-----------
 ape_test  | 0/3D583B0
```

- Check if publication exists
- By default, the pubname will be "[slot_name]_publication_for_all_tables", if you already have a publication for all tables, for example: my_some_publication, you can reuse it without creating a new one, just configure it in task_config.ini as described later in this article.
```
SELECT * FROM pg_catalog.pg_publication WHERE pubname = 'ape_dts_publication_for_all_tables';
```

- Create publication for all tables
```
CREATE PUBLICATION ape_dts_publication_for_all_tables FOR ALL TABLES;
```

- Update cdc_task_config.ini
```
[extractor]
start_lsn=0/3D583B0
pub_name=ape_dts_publication_for_all_tables
```

- Start snapshot task
- Start cdc task once snapshot task finished, all changes made during snapshot task will be synchronized to target.

# Source: Mongo
Refer to [mongo -> mongo](./mongo_to_mongo.md) to generate snapshot_task_config.ini & cdc_task_config.ini.

- Get current timestamp accurate to seconds from source Mongo
```
docker exec -it src-mongo mongosh --quiet

print(Math.floor(Date.now() / 1000));
```
```
1729070711
```

- Update cdc_task_config.ini
- This works for both source=change_stream and source=op_log
```
[extractor]
start_timestamp=1729070711
```

- Start snapshot task
- Start cdc task once snapshot task finished, all changes made during snapshot task will be synchronized to target.
---


# COMPREHENSIVE TESTING STRATEGY

## Test Coverage Goals

### 1. Unit Testing (UT) - 100% Coverage
- **Target**: All modules, functions, and methods
- **Scope**: 
  - dt-main: CLI parsing, configuration loading, entry points
  - dt-precheck: Validation logic, error detection
  - dt-connector: Database connectors (MySQL, PostgreSQL, MongoDB, Redis, Kafka, etc.)
  - dt-pipeline: Data pipeline transformations
  - dt-parallelizer: Parallel algorithms
  - dt-task: Task creation and execution logic
  - dt-common: Utility functions, data structures, metadata management

### 2. Integration Testing - 100% Coverage
- **Database Integration Tests**:
  - MySQL ↔ MySQL (snapshot, CDC, check, revise, review)
  - PostgreSQL ↔ PostgreSQL (snapshot, CDC, check, revise, review)
  - MongoDB ↔ MongoDB (snapshot, CDC, check, revise, review)
  - Redis ↔ Redis (snapshot, CDC)
  - MySQL → StarRocks (snapshot, CDC, structure migration)
  - MySQL → Clickhouse (snapshot, CDC, structure migration)
  - MySQL → Doris (snapshot, CDC, structure migration)
  - MySQL → TiDB (snapshot, CDC, structure migration)
  - PostgreSQL → StarRocks (snapshot, CDC, structure migration)
  - PostgreSQL → Clickhouse (snapshot, CDC, structure migration)
  - PostgreSQL → Doris (snapshot, CDC, structure migration)
  - MySQL → Kafka (snapshot, CDC)
  - PostgreSQL → Kafka (snapshot, CDC)

- **Feature Integration Tests**:
  - Resume from breakpoint (snapshot + CDC)
  - Heartbeat mechanisms
  - Two-way synchronization
  - Lua script data transformations
  - Parallel table processing
  - Filtering and routing (database, table, column levels)
  - Structure migration with Liquibase
  - Metrics collection (Prometheus)
  - Monitor info and position tracking

### 3. End-to-End (E2E) Testing - 100% Coverage
- **Complete Workflows**:
  - Full migration pipelines (snapshot → CDC → verification)
  - Multi-database scenarios
  - Failure recovery scenarios
  - Performance benchmarking
  - Data consistency verification
  - Long-running stability tests

## Testing Infrastructure Requirements

### 1. Test Harness Setup
- Docker Compose environments for all database types
- Automated database provisioning and teardown
- Test data generation utilities
- Performance measurement tools
- Coverage reporting integration

### 2. CI/CD Integration
- Automated test execution on every commit
- Parallel test execution
- Coverage gates (fail if coverage drops below 100%)
- Performance regression detection
- Integration with existing test suite

### 3. Test Data Management
- Synthetic data generators for all database types
- Schema migration test sets
- Edge case data (nulls, special characters, large values)
- Performance stress test datasets

## Implementation Tasks

### Phase 1: Unit Test Infrastructure
1. Set up Rust test framework with coverage tools (cargo-tarpaulin or llvm-cov)
2. Create test utilities and mocks for database connections
3. Establish baseline coverage measurement
4. Implement unit tests for dt-common (foundational)
5. Implement unit tests for dt-precheck
6. Implement unit tests for dt-connector (all database types)
7. Implement unit tests for dt-pipeline
8. Implement unit tests for dt-parallelizer
9. Implement unit tests for dt-task
10. Implement unit tests for dt-main

### Phase 2: Integration Test Infrastructure
1. Set up Docker Compose for MySQL test environments
2. Set up Docker Compose for PostgreSQL test environments
3. Set up Docker Compose for MongoDB test environments
4. Set up Docker Compose for Redis test environments
5. Set up Docker Compose for StarRocks test environments
6. Set up Docker Compose for Clickhouse test environments
7. Set up Docker Compose for Doris test environments
8. Set up Docker Compose for TiDB test environments
9. Set up Docker Compose for Kafka test environments
10. Create integration test framework and utilities

### Phase 3: Database Integration Tests
1. MySQL ↔ MySQL snapshot tests
2. MySQL ↔ MySQL CDC tests
3. MySQL ↔ MySQL check/revise/review tests
4. PostgreSQL ↔ PostgreSQL snapshot tests
5. PostgreSQL ↔ PostgreSQL CDC tests
6. PostgreSQL ↔ PostgreSQL check/revise/review tests
7. MongoDB ↔ MongoDB snapshot tests
8. MongoDB ↔ MongoDB CDC tests
9. MongoDB ↔ MongoDB check/revise/review tests
10. Redis ↔ Redis snapshot tests
11. Redis ↔ Redis CDC tests
12. MySQL → StarRocks tests (all task types)
13. MySQL → Clickhouse tests (all task types)
14. MySQL → Doris tests (all task types)
15. MySQL → TiDB tests (all task types)
16. PostgreSQL → StarRocks tests (all task types)
17. PostgreSQL → Clickhouse tests (all task types)
18. PostgreSQL → Doris tests (all task types)
19. MySQL → Kafka tests
20. PostgreSQL → Kafka tests

### Phase 4: Feature Integration Tests
1. Resume from breakpoint (snapshot) tests
2. Resume from breakpoint (CDC) tests
3. Heartbeat mechanism tests
4. Two-way synchronization tests
5. Lua script transformation tests
6. Parallel table processing tests
7. Database-level filtering tests
8. Table-level filtering tests
9. Column-level filtering tests
10. Structure migration tests
11. Liquibase integration tests
12. Metrics collection tests
13. Monitor info tests
14. Position tracking tests

### Phase 5: E2E Test Infrastructure
1. Set up E2E test orchestration framework
2. Create E2E test data generators
3. Implement performance measurement utilities
4. Create data consistency verification tools
5. Set up long-running test infrastructure

### Phase 6: E2E Test Scenarios
1. Full MySQL migration pipeline (snapshot + CDC + verify)
2. Full PostgreSQL migration pipeline
3. Full MongoDB migration pipeline
4. Full Redis migration pipeline
5. MySQL → StarRocks pipeline
6. MySQL → Clickhouse pipeline
7. MySQL → Doris pipeline
8. MySQL → TiDB pipeline
9. PostgreSQL → StarRocks pipeline
10. PostgreSQL → Clickhouse pipeline
11. PostgreSQL → Doris pipeline
12. MySQL → Kafka → Consumer pipeline
13. PostgreSQL → Kafka → Consumer pipeline
14. Multi-database mixed scenario tests
15. Failure injection and recovery tests
16. Network partition recovery tests
17. Source database restart tests
18. Target database restart tests
19. Performance benchmark regression tests
20. Data consistency verification tests
21. Long-running stability tests (24h+)

### Phase 7: CI/CD & Automation
1. Integrate all tests into CI pipeline
2. Set up parallel test execution
3. Configure coverage reporting
4. Set up coverage gates (100% threshold)
5. Implement performance regression detection
6. Create automated test result dashboards
7. Set up test failure notifications
8. Implement flaky test detection and retry logic
9. Create test execution time optimization
10. Document test execution and debugging procedures

### Phase 8: Continuous Testing
1. Monitor test coverage continuously
2. Add tests for all new features
3. Maintain test infrastructure
4. Update test data as schemas evolve
5. Performance benchmark tracking
6. Test execution optimization
7. Regular test suite audit and cleanup
8. Keep documentation up-to-date

## Success Criteria

1. **Unit Test Coverage**: 100% line coverage across all modules
2. **Integration Test Coverage**: All database combinations and features tested
3. **E2E Test Coverage**: All documented workflows have automated E2E tests
4. **CI/CD**: All tests run automatically on every commit
5. **Performance**: Test suite completes in under 30 minutes
6. **Stability**: <1% flaky test rate
7. **Documentation**: All tests are documented and maintainable

## Continuous Improvement

- Weekly test coverage reviews
- Monthly test suite performance optimization
- Quarterly test infrastructure upgrades
- Regular addition of edge case tests
- Performance regression tracking
- Test maintenance and refactoring

