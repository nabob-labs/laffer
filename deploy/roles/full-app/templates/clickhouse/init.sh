#!/bin/bash

# Create database
clickhouse-client --password="{{ clickhouse_password }}" -q "CREATE DATABASE IF NOT EXISTS {{ clickhouse_database }}"

# Create user
clickhouse-client --password="{{ clickhouse_password }}" -q "CREATE USER IF NOT EXISTS {{ clickhouse_velox_user }} IDENTIFIED BY '{{ clickhouse_velox_password }}'"

# Grant permissions
clickhouse-client --password="{{ clickhouse_password }}" -q "GRANT ALL ON {{ clickhouse_database }}.* TO {{ clickhouse_velox_user }}"
