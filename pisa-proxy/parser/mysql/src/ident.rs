// Copyright 2022 SphereEx Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

macro_rules! keyword_size {
    () => {
        // Sort by alphabetical order, use command `LC_COLLATE=C sort`
        const KEYWORD: &[&str] = &[
            "ACCOUNT",
            "ACTION",
            "ACTIVE",
            "ADD",
            "ADDDATE",
            "ADMIN",
            "AFTER",
            "AGAINST",
            "AGGREGATE",
            "ALGORITHM",
            "ALL",
            "ALWAYS",
            "AND",
            "AND_AND",
            "AND_OP",
            "ANY",
            "ARRAY",
            "AS",
            "ASC",
            "ASCII",
            "ASSIGN_GTIDS_TO_ANONYMOUS_TRANSACTIONS",
            "ASTERISK",
            "AT",
            "ATTRIBUTE",
            "AUTHENTICATION",
            "AUTOEXTEND_SIZE",
            "AUTO_INCREMENT",
            "AVG",
            "AVG_ROW_LENGTH",
            "BACKUP",
            "BEGIN",
            "BETWEEN",
            "BIGINT",
            "BINARY",
            "BINLOG",
            "BIN_NUM",
            "BIT",
            "BIT_AND",
            "BIT_OR",
            "BIT_XOR",
            "BLOB",
            "BLOCK",
            "BOOL",
            "BOOLEAN",
            "BOTH",
            "BTREE",
            "BUCKETS",
            "BY",
            "BYTE",
            "CACHE",
            "CASCADED",
            "CASE",
            "CAST",
            "CATALOG_NAME",
            "CHAIN",
            "CHALLENGE_RESPONSE",
            "CHANGED",
            "CHANNEL",
            "CHAR",
            "CHARACTER",
            "CHARSET",
            "CHECKSUM",
            "CIPHER",
            "CLASS_ORIGIN",
            "CLIENT",
            "CLONE",
            "CLOSE",
            "COALESCE",
            "CODE",
            "COLLATE",
            "COLLATION",
            "COLUMNS",
            "COLUMN_FORMAT",
            "COLUMN_NAME",
            "COMMA",
            "COMMENT",
            "COMMIT",
            "COMMITTED",
            "COMPACT",
            "COMPLETION",
            "COMPONENT",
            "COMPRESSED",
            "COMPRESSION",
            "CONCURRENT",
            "CONDITIONLESS_JOIN",
            "CONNECTION",
            "CONSISTENT",
            "CONSTRAINT_CATALOG",
            "CONSTRAINT_NAME",
            "CONSTRAINT_SCHEMA",
            "CONTAINS",
            "CONTEXT",
            "CONVERT",
            "COUNT",
            "CPU",
            "CREATE",
            "CROSS",
            "CUME_DIST",
            "CURDATE",
            "CURRENT",
            "CURRENT_USER",
            "CURSOR_NAME",
            "CURTIME",
            "DASH",
            "DATA",
            "DATABASE",
            "DATABASES",
            "DATAFILE",
            "DATE",
            "DATETIME",
            "DATE_ADD",
            "DATE_SUB",
            "DAY",
            "DAY_HOUR",
            "DAY_MICROSECOND",
            "DAY_MINUTE",
            "DAY_SECOND",
            "DEALLOCATE",
            "DECIMAL",
            "DECIMAL_NUM",
            "DEFAULT",
            "DEFAULT_AUTH",
            "DEFINER",
            "DEFINITION",
            "DELAYED",
            "DELAY_KEY_WRITE",
            "DELETE",
            "DENSE_RANK",
            "DERIVED_MERGE_HINT",
            "DESC",
            "DESCRIPTION",
            "DIAGNOSTICS",
            "DIRECTORY",
            "DISABLE",
            "DISCARD",
            "DISK",
            "DISTINCT",
            "DISTINCTROW",
            "DIV",
            "DO",
            "DOT",
            "DOUBLE",
            "DUAL",
            "DUMPFILE",
            "DUPLICATE",
            "DYNAMIC",
            "ELSE",
            "EMPTY",
            "EMPTY_FROM_CLAUSE",
            "ENABLE",
            "ENCLOSED",
            "ENCRYPTION",
            "END",
            "ENDS",
            "ENFORCED",
            "ENGINE",
            "ENGINES",
            "ENGINE_ATTRIBUTE",
            "ENUM",
            "ERROR",
            "ERRORS",
            "ESCAPE",
            "ESCAPED",
            "EVENT",
            "EVENTS",
            "EVERY",
            "EXCHANGE",
            "EXCLAMARK",
            "EXCLUDE",
            "EXECUTE",
            "EXISTS",
            "EXPANSION",
            "EXPIRE",
            "EXPORT",
            "EXTENDED",
            "EXTENT_SIZE",
            "EXTRACT",
            "FACTOR",
            "FAILED_LOGIN_ATTEMPTS",
            "FALSE",
            "FAST",
            "FAULTS",
            "FIELDS",
            "FILE",
            "FILE_BLOCK_SIZE",
            "FILTER",
            "FINISH",
            "FIRST",
            "FIRST_VALUE",
            "FIXED",
            "FLOAT",
            "FLOAT_NUM",
            "FLUSH",
            "FOLLOWING",
            "FOLLOWS",
            "FOR",
            "FORCE",
            "FOREIGN",
            "FORMAT",
            "FOUND",
            "FROM",
            "FULL",
            "FULLTEXT",
            "FUNCTION",
            "GENERAL",
            "GEOMETRY",
            "GEOMETRYCOLLECTION",
            "GET_FORMAT",
            "GET_MASTER_PUBLIC_KEY",
            "GET_SOURCE_PUBLIC_KEY",
            "GLOBAL",
            "GRANTS",
            "GROUP",
            "GROUPING",
            "GROUPS",
            "GROUP_CONCAT",
            "GROUP_REPLICATION",
            "GTID_ONLY",
            "HANDLER",
            "HASH",
            "HAVING",
            "HELP",
            "HEX_NUM",
            "HIGH_PRIORITY",
            "HISTOGRAM",
            "HISTORY",
            "HOST",
            "HOSTS",
            "HOUR",
            "HOUR_MICROSECOND",
            "HOUR_MINUTE",
            "HOUR_SECOND",
            "IDENT",
            "IDENTIFIED",
            "IDENT_QUOTED",
            "IF",
            "IGNORE",
            "IGNORE_SERVER_IDS",
            "IMPORT",
            "IN",
            "INACTIVE",
            "INDEX",
            "INDEXES",
            "INITIAL",
            "INITIAL_SIZE",
            "INITIATE",
            "INNER",
            "INSERT",
            "INSERT_METHOD",
            "INSTALL",
            "INSTANCE",
            "INT",
            "INTEGER",
            "INTERVAL",
            "INTO",
            "INVISIBLE",
            "INVOKER",
            "IO",
            "IPC",
            "IS",
            "ISOLATION",
            "ISSUER",
            "JOIN",
            "JSON",
            "JSON_ARRAYAGG",
            "JSON_OBJECTAGG",
            "JSON_SEPARATOR",
            "JSON_TABLE",
            "JSON_UNQUOTED_SEPARATOR",
            "JSON_VALUE",
            "KEY",
            "KEYRING",
            "KEYS",
            "KEY_BLOCK_SIZE",
            "LAG",
            "LANGUAGE",
            "LAST",
            "LAST_VALUE",
            "LATERAL",
            "LBRACE",
            "LEAD",
            "LEADING",
            "LEAVES",
            "LEFT",
            "LESS",
            "LEVEL",
            "LIKE",
            "LIMIT",
            "LINEAR",
            "LINES",
            "LINESTRING",
            "LIST",
            "LOCAL",
            "LOCK",
            "LOCKED",
            "LOCKS",
            "LOGFILE",
            "LOGS",
            "LONG",
            "LONGBLOB",
            "LONGTEXT",
            "LONG_NUM",
            "LOW_PRIORITY",
            "LPAREN",
            "MASTER",
            "MASTER_AUTO_POSITION",
            "MASTER_COMPRESSION_ALGORITHM",
            "MASTER_CONNECT_RETRY",
            "MASTER_DELAY",
            "MASTER_HEARTBEAT_PERIOD",
            "MASTER_HOST",
            "MASTER_LOG_FILE",
            "MASTER_LOG_POS",
            "MASTER_PASSWORD",
            "MASTER_PORT",
            "MASTER_PUBLIC_KEY_PATH",
            "MASTER_RETRY_COUNT",
            "MASTER_SSL",
            "MASTER_SSL_CA",
            "MASTER_SSL_CAPATH",
            "MASTER_SSL_CERT",
            "MASTER_SSL_CIPHER",
            "MASTER_SSL_CRL",
            "MASTER_SSL_CRLPATH",
            "MASTER_SSL_KEY",
            "MASTER_TLS_CIPHERSUITES",
            "MASTER_TLS_VERSION",
            "MASTER_USER",
            "MASTER_ZSTD_COMPRESSION_LEVEL",
            "MATCH",
            "MAX",
            "MAX_CONNECTIONS_PER_HOUR",
            "MAX_QUERIES_PER_HOUR",
            "MAX_ROWS",
            "MAX_SIZE",
            "MAX_UPDATES_PER_HOUR",
            "MAX_USER_CONNECTIONS",
            "MAXVALUE",
            "MEDIUM",
            "MEDIUMBLOB",
            "MEDIUMINT",
            "MEDIUMTEXT",
            "MEMBER",
            "MEMORY",
            "MESSAGE_TEXT",
            "MICROSECOND",
            "MIGRATE",
            "MIN",
            "MINUTE",
            "MINUTE_MICROSECOND",
            "MINUTE_SECOND",
            "MIN_ROWS",
            "MOD",
            "MODE",
            "MODIFY",
            "MONTH",
            "MULTILINESTRING",
            "MULTIPOINT",
            "MULTIPOLYGON",
            "MUTEX",
            "MYSQL_ERRNO",
            "NAME",
            "NAMES",
            "NATIONAL",
            "NATURAL",
            "NCHAR",
            "NCHAR_STRING",
            "NDBCLUSTER_",
            "NESTED",
            "NETWORK_NAMESPACE",
            "NEVER",
            "NEW",
            "NEXT",
            "NO",
            "NODEGROUP",
            "NONE",
            "NOT",
            "NOT2",
            "NOW",
            "NOWAIT",
            "NO_WAIT",
            "NTH_VALUE",
            "NTILE",
            "NULL",
            "NULLS",
            //"NUM",
            "NUMBER",
            "NUMERIC",
            "NVARCHAR",
            "OF",
            "OFF",
            "OFFSET",
            "OJ",
            "OLD",
            "ON",
            "ONE",
            "ONLY",
            "OPEN",
            "OPTIONAL",
            "OPTIONALLY",
            "OPTIONS",
            "OR",
            "ORDER",
            "ORDINALITY",
            "ORGANIZATION",
            "OTHERS",
            "OUTER",
            "OUTFILE",
            "OVER",
            "OWNER",
            "PACK_KEYS",
            "PAGE",
            "PARAM_MARKER",
            "PARSER",
            "PARTIAL",
            "PARTITION",
            "PARTITIONING",
            "PARTITIONS",
            "PASSWORD",
            "PASSWORD_LOCK_TIME",
            "PATH",
            "PERCENT",
            "PERCENT_RANK",
            "PERSIST",
            "PERSIST_ONLY",
            "PHASE",
            "PLUGIN",
            "PLUGINS",
            "PLUGIN_DIR",
            "PLUS",
            "POINT",
            "POLYGON",
            "PORT",
            "POSITION",
            "PRECEDES",
            "PRECEDING",
            "PRECISION",
            "PREPARE",
            "PRESERVE",
            "PREV",
            "PRIMARY",
            "PRIVILEGES",
            "PRIVILEGE_CHECKS_USER",
            "PROCEDURE",
            "PROCESS",
            "PROCESSLIST",
            "PROFILE",
            "PROFILES",
            "PROXY",
            "QUARTER",
            "QUERY",
            "QUICK",
            "RANDOM",
            "RANGE",
            "RANK",
            "RBRACE",
            "READ",
            "READ_ONLY",
            "REAL",
            "REBUILD",
            "RECOVER",
            "RECURSIVE",
            "REDO_BUFFER_SIZE",
            "REDUNDANT",
            "REFERENCE",
            "REGEXP",
            "REGISTRATION",
            "RELAY",
            "RELAYLOG",
            "RELAY_LOG_FILE",
            "RELAY_LOG_POS",
            "RELAY_THREAD",
            "RELOAD",
            "REMOVE",
            "REORGANIZE",
            "REPAIR",
            "REPEAT",
            "REPEATABLE",
            "REPLACE",
            "REPLICA",
            "REPLICAS",
            "REPLICATE_DO_DB",
            "REPLICATE_DO_TABLE",
            "REPLICATE_IGNORE_DB",
            "REPLICATE_IGNORE_TABLE",
            "REPLICATE_REWRITE_DB",
            "REPLICATE_WILD_DO_TABLE",
            "REPLICATE_WILD_IGNORE_TABLE",
            "REPLICATION",
            "REQUIRE",
            "REQUIRE_ROW_FORMAT",
            "REQUIRE_TABLE_PRIMARY_KEY_CHECK",
            "RESET",
            "RESOURCE",
            "RESOURCES",
            "RESPECT",
            "RESTART",
            "RESTORE",
            "RESUME",
            "RETAIN",
            "RETURNED_SQLSTATE",
            "RETURNING",
            "RETURNS",
            "REUSE",
            "REVERSE",
            "RIGHT",
            "RLIKE",
            "ROLE",
            "ROLLBACK",
            "ROLLUP",
            "ROTATE",
            "ROUTINE",
            "ROW",
            "ROWS",
            "ROW_COUNT",
            "ROW_FORMAT",
            "ROW_NUMBER",
            "RPAREN",
            "RTREE",
            "SAVEPOINT",
            "SCHEDULE",
            "SCHEMA_NAME",
            "SECOND",
            "SECONDARY",
            "SECONDARY_ENGINE",
            "SECONDARY_ENGINE_ATTRIBUTE",
            "SECONDARY_LOAD",
            "SECONDARY_UNLOAD",
            "SECOND_MICROSECOND",
            "SECURITY",
            "SELECT",
            "SEMICOLON",
            "SEPARATOR",
            "SERIAL",
            "SERIALIZABLE",
            "SERVER",
            "SESSION",
            "SET",
            "SET_VAR_HINT",
            "SHARE",
            "SHOW",
            "SHUTDOWN",
            "SIGNED",
            "SIMPLE",
            "SKIP",
            "SLASH",
            "SLAVE",
            "SLOW",
            "SMALLINT",
            "SNAPSHOT",
            "SOCKET",
            "SOME",
            "SONAME",
            "SOUNDS",
            "SOURCE",
            "SOURCE_AUTO_POSITION",
            "SOURCE_BIND",
            "SOURCE_COMPRESSION_ALGORITHM",
            "SOURCE_CONNECTION_AUTO_FAILOVER",
            "SOURCE_CONNECT_RETRY",
            "SOURCE_DELAY",
            "SOURCE_HEARTBEAT_PERIOD",
            "SOURCE_HOST",
            "SOURCE_LOG_FILE",
            "SOURCE_LOG_POS",
            "SOURCE_PASSWORD",
            "SOURCE_PORT",
            "SOURCE_PUBLIC_KEY_PATH",
            "SOURCE_RETRY_COUNT",
            "SOURCE_SSL",
            "SOURCE_SSL_CA",
            "SOURCE_SSL_CAPATH",
            "SOURCE_SSL_CERT",
            "SOURCE_SSL_CIPHER",
            "SOURCE_SSL_CRL",
            "SOURCE_SSL_CRLPATH",
            "SOURCE_SSL_KEY",
            "SOURCE_SSL_VERIFY_SERVER_CERT",
            "SOURCE_TLS_CIPHERSUITES",
            "SOURCE_TLS_VERSION",
            "SOURCE_USER",
            "SOURCE_ZSTD_COMPRESSION_LEVEL",
            "SPATIAL",
            "SQL_AFTER_GTIDS",
            "SQL_AFTER_MTS_GAPS",
            "SQL_BEFORE_GTIDS",
            "SQL_BIG_RESULT",
            "SQL_BUFFER_RESULT",
            "SQL_CACHE",
            "SQL_CALC_FOUND_ROWS",
            "SQL_NO_CACHE",
            "SQL_SMALL_RESULT",
            "SQL_THREAD",
            "SRID",
            "SSL",
            "STACKED",
            "START",
            "STARTING",
            "STARTS",
            "STATS_AUTO_RECALC",
            "STATS_PERSISTENT",
            "STATS_SAMPLE_PAGES",
            "STATUS",
            "STD",
            "STDDEV",
            "STDDEV_POP",
            "STDDEV_SAMP",
            "STOP",
            "STORAGE",
            "STORED",
            "STRAIGHT_JOIN",
            "STREAM",
            "ST_COLLECT",
            "SUBCLASS_ORIGIN",
            "SUBDATE",
            "SUBJECT",
            "SUBPARTITION",
            "SUBPARTITIONS",
            "SUBQUERY_AS_EXPR",
            "SUBSTRING",
            "SUM",
            "SUPER",
            "SUSPEND",
            "SWAPS",
            "SWITCHES",
            "SYSDATE",
            "TABLE",
            "TABLES",
            "TABLESPACE",
            "TABLE_CHECKSUM",
            "TABLE_NAME",
            "TEMPORARY",
            "TEMPTABLE",
            "TERMINATED",
            "TEXT",
            "TEXT_STRING",
            "THAN",
            "THEN",
            "THREAD_PRIORITY",
            "TIES",
            "TIME",
            "TIMESTAMP",
            "TIMESTAMP_ADD",
            "TIMESTAMP_DIFF",
            "TINYBLOB",
            "TINYINT",
            "TINYTEXT_SYN",
            "TLS",
            "TRAILING",
            "TRANSACTION",
            "TRIGGER",
            "TRIGGERS",
            "TRIM",
            "TRUE",
            "TRUNCATE",
            "TYPE",
            "TYPES",
            "ULONGLONG_NUM",
            "UNBOUNDED",
            "UNCOMMITTED",
            "UNDEFINED",
            "UNDERSCORE_CHARSET",
            "UNDO",
            "UNDOFILE",
            "UNDO_BUFFER_SIZE",
            "UNICODE",
            "UNINSTALL",
            "UNION",
            "UNIQUE",
            "UNKNOWN",
            "UNLOCK",
            "UNREGISTER",
            "UNSIGNED",
            "UNTIL",
            "UPDATE",
            "UPGRADE",
            "USE",
            "USER",
            "USE_FRM",
            "USING",
            "UTC_DATE",
            "UTC_TIME",
            "UTC_TIMESTAMP",
            "VALIDATION",
            "VALUE",
            "VALUES",
            "VARBINARY",
            "VARCHAR",
            "VARIABLES",
            "VARIANCE",
            "VARYING",
            "VAR_POP",
            "VAR_SAMP",
            "VCPU",
            "VIEW",
            "VISIBLE",
            "WAIT",
            "WARNINGS",
            "WEEK",
            "WEIGHT_STRING",
            "WHEN",
            "WHERE",
            "WINDOW",
            "WITH",
            "WITHOUT",
            "WORK",
            "WRAPPER",
            "X509",
            "XA",
            "XID",
            "XML",
            "XOR",
            "XOR_OP",
            "YEAR",
            "YEAR_MONTH",
            "ZEROFILL",
            "ZONE",
        ];

        const KEYWORD_SIZE: &[(u32, usize)] = &[
            (T_ACCOUNT, 7),
            (T_ACTION, 6),
            (T_ACTIVE, 6),
            (T_ADD, 3),
            (T_ADDDATE, 7),
            (T_ADMIN, 5),
            (T_AFTER, 5),
            (T_AGAINST, 7),
            (T_AGGREGATE, 9),
            (T_ALGORITHM, 9),
            (T_ALL, 3),
            (T_ALWAYS, 6),
            (T_AND, 3),
            (T_AND_AND, 2),
            (T_AND_OP, 1),
            (T_ANY, 3),
            (T_ARRAY, 5),
            (T_AS, 2),
            (T_ASC, 3),
            (T_ASCII, 5),
            (T_ASSIGN_GTIDS_TO_ANONYMOUS_TRANSACTIONS, 38),
            (T_ASTERISK, 8),
            (T_AT_SYM, 2),
            (T_ATTRIBUTE, 9),
            (T_AUTHENTICATION, 14),
            (T_AUTOEXTEND_SIZE, 15),
            (T_AUTO_INCREMENT, 14),
            (T_AVG, 3),
            (T_AVG_ROW_LENGTH, 14),
            (T_BACKUP, 6),
            (T_BEGIN, 5),
            (T_BETWEEN, 7),
            (T_BIGINT, 6),
            (T_BINARY, 6),
            (T_BINLOG, 6),
            (T_BIN_NUM, 7),
            (T_BIT, 3),
            (T_BIT_AND, 7),
            (T_BIT_OR, 1),
            (T_BIT_XOR, 1),
            (T_BLOB, 4),
            (T_BLOCK, 5),
            (T_BOOL, 4),
            (T_BOOLEAN, 7),
            (T_BOTH, 4),
            (T_BTREE, 5),
            (T_BUCKETS, 7),
            (T_BY, 2),
            (T_BYTE, 4),
            (T_CACHE, 5),
            (T_CASCADED, 8),
            (T_CASE, 4),
            (T_CAST, 4),
            (T_CATALOG_NAME, 12),
            (T_CHAIN, 5),
            (T_CHALLENGE_RESPONSE, 18),
            (T_CHANGED, 7),
            (T_CHANNEL, 7),
            (T_CHAR, 4),
            (T_CHARACTER, 9),
            (T_CHARSET, 7),
            (T_CHECKSUM, 8),
            (T_CIPHER, 6),
            (T_CLASS_ORIGIN, 12),
            (T_CLIENT, 6),
            (T_CLONE, 5),
            (T_CLOSE, 5),
            (T_COALESCE, 8),
            (T_CODE, 4),
            (T_COLLATE, 7),
            (T_COLLATION, 9),
            (T_COLUMNS, 7),
            (T_COLUMN_FORMAT, 13),
            (T_COLUMN_NAME, 11),
            (T_COMMA, 5),
            (T_COMMENT, 7),
            (T_COMMIT, 6),
            (T_COMMITTED, 9),
            (T_COMPACT, 7),
            (T_COMPLETION, 10),
            (T_COMPONENT, 9),
            (T_COMPRESSED, 10),
            (T_COMPRESSION, 11),
            (T_CONCURRENT, 10),
            (T_CONDITIONLESS_JOIN, 18),
            (T_CONNECTION, 10),
            (T_CONSISTENT, 10),
            (T_CONSTRAINT_CATALOG, 18),
            (T_CONSTRAINT_NAME, 15),
            (T_CONSTRAINT_SCHEMA, 17),
            (T_CONTAINS, 8),
            (T_CONTEXT, 7),
            (T_CONVERT, 7),
            (T_COUNT, 5),
            (T_CPU, 3),
            (T_CREATE, 6),
            (T_CROSS, 5),
            (T_CUME_DIST, 9),
            (T_CURDATE, 7),
            (T_CURRENT, 7),
            (T_CURRENT_USER, 12),
            (T_CURSOR_NAME, 11),
            (T_CURTIME, 7),
            (T_DASH, 4),
            (T_DATA, 4),
            (T_DATABASE, 8),
            (T_DATABASES, 9),
            (T_DATAFILE, 8),
            (T_DATE, 4),
            (T_DATETIME, 8),
            (T_DATE_ADD, 8),
            (T_DATE_SUB, 8),
            (T_DAY, 3),
            (T_DAY_HOUR, 8),
            (T_DAY_MICROSECOND, 15),
            (T_DAY_MINUTE, 10),
            (T_DAY_SECOND, 10),
            (T_DEALLOCATE, 10),
            (T_DECIMAL, 8),
            (T_DECIMAL_NUM, 11),
            (T_DEFAULT, 7),
            (T_DEFAULT_AUTH, 12),
            (T_DEFINER, 7),
            (T_DEFINITION, 10),
            (T_DELAYED, 7),
            (T_DELAY_KEY_WRITE, 15),
            (T_DELETE, 6),
            (T_DENSE_RANK, 10),
            (T_DERIVED_MERGE_HINT, 18),
            (T_DESC, 4),
            (T_DESCRIPTION, 11),
            (T_DIAGNOSTICS, 11),
            (T_DIRECTORY, 9),
            (T_DISABLE, 7),
            (T_DISCARD, 7),
            (T_DISK, 4),
            (T_DISTINCT, 8),
            (T_DISTINCTROW, 11),
            (T_DIV, 3),
            (T_DO, 2),
            (T_DOT, 3),
            (T_DOUBLE, 6),
            (T_DUAL, 4),
            (T_DUMPFILE, 8),
            (T_DUPLICATE, 9),
            (T_DYNAMIC, 7),
            (T_ELSE, 4),
            (T_EMPTY, 5),
            (T_EMPTY_FROM_CLAUSE, 17),
            (T_ENABLE, 6),
            (T_ENCLOSED, 8),
            (T_ENCRYPTION, 10),
            (T_END, 3),
            (T_ENDS, 4),
            (T_ENFORCED, 8),
            (T_ENGINE, 6),
            (T_ENGINES, 7),
            (T_ENGINE_ATTRIBUTE, 16),
            (T_ENUM, 4),
            (T_ERROR, 5),
            (T_ERRORS, 6),
            (T_ESCAPE, 6),
            (T_ESCAPED, 7),
            (T_EVENT, 5),
            (T_EVENTS, 6),
            (T_EVERY, 5),
            (T_EXCHANGE, 8),
            (T_EXCLAMARK, 9),
            (T_EXCLUDE, 7),
            (T_EXECUTE, 7),
            (T_EXISTS, 6),
            (T_EXPANSION, 9),
            (T_EXPIRE, 6),
            (T_EXPORT, 6),
            (T_EXTENDED, 8),
            (T_EXTENT_SIZE, 11),
            (T_EXTRACT, 7),
            (T_FACTOR, 6),
            (T_FAILED_LOGIN_ATTEMPTS, 21),
            (T_FALSE, 5),
            (T_FAST, 4),
            (T_FAULTS, 6),
            (T_FIELDS, 6),
            (T_FILE, 4),
            (T_FILE_BLOCK_SIZE, 15),
            (T_FILTER, 6),
            (T_FINISH, 6),
            (T_FIRST, 5),
            (T_FIRST_VALUE, 11),
            (T_FIXED, 5),
            (T_FLOAT, 5),
            (T_FLOAT_NUM, 9),
            (T_FLUSH, 5),
            (T_FOLLOWING, 9),
            (T_FOLLOWS, 7),
            (T_FOR, 3),
            (T_FORCE, 5),
            (T_FOREIGN, 7),
            (T_FORMAT, 6),
            (T_FOUND, 5),
            (T_FROM, 4),
            (T_FULL, 4),
            (T_FULLTEXT, 8),
            (T_FUNCTION, 8),
            (T_GENERAL, 7),
            (T_GEOMETRY, 8),
            (T_GEOMETRYCOLLECTION, 18),
            (T_GET_FORMAT, 10),
            (T_GET_MASTER_PUBLIC_KEY, 21),
            (T_GET_SOURCE_PUBLIC_KEY, 21),
            (T_GLOBAL, 6),
            (T_GRANTS, 6),
            (T_GROUP, 5),
            (T_GROUPING, 8),
            (T_GROUPS, 6),
            (T_GROUP_CONCAT, 12),
            (T_GROUP_REPLICATION, 17),
            (T_GTID_ONLY, 9),
            (T_HANDLER, 7),
            (T_HASH, 4),
            (T_HAVING, 6),
            (T_HELP, 4),
            (T_HEX_NUM, 7),
            (T_HIGH_PRIORITY, 13),
            (T_HISTOGRAM, 9),
            (T_HISTORY, 7),
            (T_HOST, 4),
            (T_HOSTS, 5),
            (T_HOUR, 4),
            (T_HOUR_MICROSECOND, 16),
            (T_HOUR_MINUTE, 11),
            (T_HOUR_SECOND, 11),
            (T_IDENT, 5),
            (T_IDENTIFIED, 10),
            (T_IDENT_QUOTED, 12),
            (T_IF, 2),
            (T_IGNORE, 6),
            (T_IGNORE_SERVER_IDS, 17),
            (T_IMPORT, 6),
            (T_IN, 2),
            (T_INACTIVE, 8),
            (T_INDEX, 5),
            (T_INDEXES, 7),
            (T_INITIAL, 7),
            (T_INITIAL_SIZE, 12),
            (T_INITIATE, 8),
            (T_INNER, 5),
            (T_INSERT, 6),
            (T_INSERT_METHOD, 13),
            (T_INSTALL, 7),
            (T_INSTANCE, 8),
            (T_INT, 3),
            (T_INT, 7),
            (T_INTERVAL, 8),
            (T_INTO, 4),
            (T_INVISIBLE, 9),
            (T_INVOKER, 7),
            (T_IO, 2),
            (T_IPC, 3),
            (T_IS, 2),
            (T_ISOLATION, 9),
            (T_ISSUER, 6),
            (T_JOIN, 4),
            (T_JSON, 4),
            (T_JSON_ARRAYAGG, 13),
            (T_JSON_OBJECTAGG, 14),
            (T_JSON_SEPARATOR, 14),
            (T_JSON_TABLE, 10),
            (T_JSON_UNQUOTED_SEPARATOR, 23),
            (T_JSON_VALUE, 10),
            (T_KEY, 3),
            (T_KEYRING, 7),
            (T_KEYS, 4),
            (T_KEY_BLOCK_SIZE, 14),
            (T_LAG, 3),
            (T_LANGUAGE, 8),
            (T_LAST, 4),
            (T_LAST_VALUE, 10),
            (T_LATERAL, 7),
            (T_LBRACE, 6),
            (T_LEAD, 4),
            (T_LEADING, 7),
            (T_LEAVES, 6),
            (T_LEFT, 4),
            (T_LESS, 4),
            (T_LEVEL, 5),
            (T_LIKE, 4),
            (T_LIMIT, 5),
            (T_LINEAR, 6),
            (T_LINES, 5),
            (T_LINESTRING, 10),
            (T_LIST, 4),
            (T_LOCAL, 5),
            (T_LOCK, 4),
            (T_LOCKED, 6),
            (T_LOCKS, 5),
            (T_LOGFILE, 7),
            (T_LOGS, 4),
            (T_LONG, 4),
            (T_LONGBLOB, 8),
            (T_LONGTEXT, 8),
            (T_LONG_NUM, 8),
            (T_LOW_PRIORITY, 12),
            (T_LPAREN, 6),
            (T_MASTER, 6),
            (T_MASTER_AUTO_POSITION, 20),
            (T_MASTER_COMPRESSION_ALGORITHM, 28),
            (T_MASTER_CONNECT_RETRY, 20),
            (T_MASTER_DELAY, 12),
            (T_MASTER_HEARTBEAT_PERIOD, 23),
            (T_MASTER_HOST, 11),
            (T_MASTER_LOG_FILE, 15),
            (T_MASTER_LOG_POS, 14),
            (T_MASTER_PASSWORD, 15),
            (T_MASTER_PORT, 11),
            (T_MASTER_PUBLIC_KEY_PATH, 22),
            (T_MASTER_RETRY_COUNT, 18),
            (T_MASTER_SSL, 10),
            (T_MASTER_SSL_CA, 13),
            (T_MASTER_SSL_CAPATH, 17),
            (T_MASTER_SSL_CERT, 15),
            (T_MASTER_SSL_CIPHER, 17),
            (T_MASTER_SSL_CRL, 14),
            (T_MASTER_SSL_CRLPATH, 18),
            (T_MASTER_SSL_KEY, 14),
            (T_MASTER_TLS_CIPHERSUITES, 23),
            (T_MASTER_TLS_VERSION, 18),
            (T_MASTER_USER, 11),
            (T_MASTER_ZSTD_COMPRESSION_LEVEL, 29),
            (T_MATCH, 5),
            (T_MAX, 3),
            (T_MAX_CONNECTIONS_PER_HOUR, 24),
            (T_MAX_QUERIES_PER_HOUR, 20),
            (T_MAX_ROWS, 8),
            (T_MAX_SIZE, 8),
            (T_MAX_UPDATES_PER_HOUR, 20),
            (T_MAX_USER_CONNECTIONS, 20),
            (T_MAXVALUE, 8),
            (T_MEDIUM, 6),
            (T_MEDIUMBLOB, 10),
            (T_MEDIUMINT, 9),
            (T_MEDIUMTEXT, 10),
            (T_MEMBER, 6),
            (T_MEMORY, 6),
            (T_MESSAGE_TEXT, 12),
            (T_MICROSECOND, 11),
            (T_MIGRATE, 7),
            (T_MIN, 3),
            (T_MINUTE, 6),
            (T_MINUTE_MICROSECOND, 18),
            (T_MINUTE_SECOND, 13),
            (T_MIN_ROWS, 8),
            (T_MOD, 3),
            (T_MODE, 4),
            (T_MODIFY, 6),
            (T_MONTH, 5),
            (T_MULTILINESTRING, 15),
            (T_MULTIPOINT, 10),
            (T_MULTIPOLYGON, 12),
            (T_MUTEX, 5),
            (T_MYSQL_ERRNO, 11),
            (T_NAME, 4),
            (T_NAMES, 5),
            (T_NATIONAL, 8),
            (T_NATURAL, 7),
            (T_NCHAR, 5),
            (T_NCHAR_STRING, 12),
            (T_NDBCLUSTER_, 11),
            (T_NESTED, 6),
            (T_NETWORK_NAMESPACE, 17),
            (T_NEVER, 5),
            (T_NEW, 3),
            (T_NEXT, 4),
            (T_NO, 2),
            (T_NODEGROUP, 9),
            (T_NONE, 4),
            (T_NOT, 3),
            (T_NOT2, 4),
            (T_NOW, 3),
            (T_NOWAIT, 6),
            (T_NO_WAIT, 7),
            (T_NTH_VALUE, 9),
            (T_NTILE, 5),
            (T_NULL, 4),
            (T_NULLS, 5),
            //(T_NUM,3),
            (T_NUMBER, 6),
            (T_NUMERIC, 7),
            (T_NVARCHAR, 8),
            (T_OF, 2),
            (T_OFF, 3),
            (T_OFFSET, 6),
            (T_OJ, 2),
            (T_OLD, 3),
            (T_ON, 2),
            (T_ONE, 3),
            (T_ONLY, 4),
            (T_OPEN, 4),
            (T_OPTIONAL, 8),
            (T_OPTIONALLY, 10),
            (T_OPTIONS, 7),
            (T_OR, 1),
            (T_ORDER, 5),
            (T_ORDINALITY, 10),
            (T_ORGANIZATION, 12),
            (T_OTHERS, 6),
            (T_OUTER, 5),
            (T_OUTFILE, 7),
            (T_OVER, 4),
            (T_OWNER, 5),
            (T_PACK_KEYS, 9),
            (T_PAGE, 4),
            (T_PARAM_MARKER, 12),
            (T_PARSER, 6),
            (T_PARTIAL, 7),
            (T_PARTITION, 9),
            (T_PARTITIONING, 12),
            (T_PARTITIONS, 10),
            (T_PASSWORD, 8),
            (T_PASSWORD_LOCK_TIME, 18),
            (T_PATH, 4),
            (T_PERCENT, 7),
            (T_PERCENT_RANK, 12),
            (T_PERSIST, 7),
            (T_PERSIST_ONLY, 12),
            (T_PHASE, 5),
            (T_PLUGIN, 6),
            (T_PLUGINS, 7),
            (T_PLUGIN_DIR, 10),
            (T_PLUS, 4),
            (T_POINT, 5),
            (T_POLYGON, 7),
            (T_PORT, 4),
            (T_POSITION, 8),
            (T_PRECEDES, 8),
            (T_PRECEDING, 9),
            (T_PRECISION, 9),
            (T_PREPARE, 7),
            (T_PRESERVE, 8),
            (T_PREV, 4),
            (T_PRIMARY, 7),
            (T_PRIVILEGES, 10),
            (T_PRIVILEGE_CHECKS_USER, 21),
            (T_PROCEDURE, 9),
            (T_PROCESS, 7),
            (T_PROCESSLIST, 11),
            (T_PROFILE, 7),
            (T_PROFILES, 8),
            (T_PROXY, 5),
            (T_QUARTER, 7),
            (T_QUERY, 5),
            (T_QUICK, 5),
            (T_RANDOM, 6),
            (T_RANGE, 5),
            (T_RANK, 4),
            (T_RBRACE, 6),
            (T_READ, 4),
            (T_READ_ONLY, 9),
            (T_REAL, 4),
            (T_REBUILD, 7),
            (T_RECOVER, 7),
            (T_RECURSIVE, 9),
            (T_REDO_BUFFER_SIZE, 16),
            (T_REDUNDANT, 9),
            (T_REFERENCE, 9),
            (T_REGEXP, 6),
            (T_REGISTRATION, 12),
            (T_RELAY, 5),
            (T_RELAYLOG, 8),
            (T_RELAY_LOG_FILE, 14),
            (T_RELAY_LOG_POS, 13),
            (T_RELAY_THREAD, 12),
            (T_RELOAD, 6),
            (T_REMOVE, 6),
            (T_REORGANIZE, 10),
            (T_REPAIR, 6),
            (T_REPEAT, 6),
            (T_REPEATABLE, 10),
            (T_REPLACE, 7),
            (T_REPLICA, 7),
            (T_REPLICAS, 8),
            (T_REPLICATE_DO_DB, 15),
            (T_REPLICATE_DO_TABLE, 18),
            (T_REPLICATE_IGNORE_DB, 19),
            (T_REPLICATE_IGNORE_TABLE, 22),
            (T_REPLICATE_REWRITE_DB, 20),
            (T_REPLICATE_WILD_DO_TABLE, 23),
            (T_REPLICATE_WILD_IGNORE_TABLE, 27),
            (T_REPLICATION, 11),
            (T_REQUIRE, 7),
            (T_REQUIRE_ROW_FORMAT, 18),
            (T_REQUIRE_TABLE_PRIMARY_KEY_CHECK, 31),
            (T_RESET, 5),
            (T_RESOURCE, 8),
            (T_RESOURCES, 9),
            (T_RESPECT, 7),
            (T_RESTART, 7),
            (T_RESTORE, 7),
            (T_RESUME, 6),
            (T_RETAIN, 6),
            (T_RETURNED_SQLSTATE, 17),
            (T_RETURNING, 9),
            (T_RETURNS, 7),
            (T_REUSE, 5),
            (T_REVERSE, 7),
            (T_RIGHT, 5),
            (T_REGEXP, 5), // RLIKE is the same as REGEXP
            (T_ROLE, 4),
            (T_ROLLBACK, 8),
            (T_ROLLUP, 6),
            (T_ROTATE, 6),
            (T_ROUTINE, 7),
            (T_ROW, 3),
            (T_ROWS, 4),
            (T_ROW_COUNT, 9),
            (T_ROW_FORMAT, 10),
            (T_ROW_NUMBER, 10),
            (T_RPAREN, 6),
            (T_RTREE, 5),
            (T_SAVEPOINT, 9),
            (T_SCHEDULE, 8),
            (T_SCHEMA_NAME, 11),
            (T_SECOND, 6),
            (T_SECONDARY, 9),
            (T_SECONDARY_ENGINE, 16),
            (T_SECONDARY_ENGINE_ATTRIBUTE, 26),
            (T_SECONDARY_LOAD, 14),
            (T_SECONDARY_UNLOAD, 16),
            (T_SECOND_MICROSECOND, 18),
            (T_SECURITY, 8),
            (T_SELECT, 6),
            (T_SEMICOLON, 9),
            (T_SEPARATOR, 9),
            (T_SERIAL, 6),
            (T_SERIALIZABLE, 12),
            (T_SERVER, 6),
            (T_SESSION, 7),
            (T_SET, 3),
            (T_SET_VAR_HINT, 12),
            (T_SHARE, 5),
            (T_SHOW, 4),
            (T_SHUTDOWN, 8),
            (T_SIGNED, 6),
            (T_SIMPLE, 6),
            (T_SKIP, 4),
            (T_SLASH, 5),
            (T_SLAVE, 5),
            (T_SLOW, 4),
            (T_SMALLINT, 8),
            (T_SNAPSHOT, 8),
            (T_SOCKET, 6),
            (T_SOME, 4),
            (T_SONAME, 6),
            (T_SOUNDS, 6),
            (T_SOURCE, 6),
            (T_SOURCE_AUTO_POSITION, 20),
            (T_SOURCE_BIND, 11),
            (T_SOURCE_COMPRESSION_ALGORITHM, 28),
            (T_SOURCE_CONNECTION_AUTO_FAILOVER, 31),
            (T_SOURCE_CONNECT_RETRY, 20),
            (T_SOURCE_DELAY, 12),
            (T_SOURCE_HEARTBEAT_PERIOD, 23),
            (T_SOURCE_HOST, 11),
            (T_SOURCE_LOG_FILE, 15),
            (T_SOURCE_LOG_POS, 14),
            (T_SOURCE_PASSWORD, 15),
            (T_SOURCE_PORT, 11),
            (T_SOURCE_PUBLIC_KEY_PATH, 22),
            (T_SOURCE_RETRY_COUNT, 18),
            (T_SOURCE_SSL, 10),
            (T_SOURCE_SSL_CA, 13),
            (T_SOURCE_SSL_CAPATH, 17),
            (T_SOURCE_SSL_CERT, 15),
            (T_SOURCE_SSL_CIPHER, 17),
            (T_SOURCE_SSL_CRL, 14),
            (T_SOURCE_SSL_CRLPATH, 18),
            (T_SOURCE_SSL_KEY, 14),
            (T_SOURCE_SSL_VERIFY_SERVER_CERT, 29),
            (T_SOURCE_TLS_CIPHERSUITES, 23),
            (T_SOURCE_TLS_VERSION, 18),
            (T_SOURCE_USER, 11),
            (T_SOURCE_ZSTD_COMPRESSION_LEVEL, 29),
            (T_SPATIAL, 7),
            (T_SQL_AFTER_GTIDS, 15),
            (T_SQL_AFTER_MTS_GAPS, 18),
            (T_SQL_BEFORE_GTIDS, 16),
            (T_SQL_BIG_RESULT, 14),
            (T_SQL_BUFFER_RESULT, 17),
            (T_SQL_CACHE, 9),
            (T_SQL_CALC_FOUND_ROWS, 19),
            (T_SQL_NO_CACHE, 12),
            (T_SQL_SMALL_RESULT, 16),
            (T_SQL_THREAD, 10),
            (T_SRID, 4),
            (T_SSL, 3),
            (T_STACKED, 7),
            (T_START, 5),
            (T_STARTING, 8),
            (T_STARTS, 6),
            (T_STATS_AUTO_RECALC, 17),
            (T_STATS_PERSISTENT, 16),
            (T_STATS_SAMPLE_PAGES, 18),
            (T_STATUS, 6),
            (T_STD, 3),
            (T_STDDEV, 6),
            (T_STDDEV_POP, 10),
            (T_STDDEV_SAMP, 11),
            (T_STOP, 4),
            (T_STORAGE, 7),
            (T_STORED, 6),
            (T_STRAIGHT_JOIN, 13),
            (T_STREAM, 6),
            (T_ST_COLLECT, 10),
            (T_SUBCLASS_ORIGIN, 15),
            (T_SUBDATE, 7),
            (T_SUBJECT, 7),
            (T_SUBPARTITION, 12),
            (T_SUBPARTITIONS, 13),
            (T_SUBQUERY_AS_EXPR, 16),
            (T_SUBSTRING, 9),
            (T_SUM, 3),
            (T_SUPER, 5),
            (T_SUSPEND, 7),
            (T_SWAPS, 5),
            (T_SWITCHES, 8),
            (T_SYSDATE, 7),
            (T_TABLE, 5),
            (T_TABLES, 6),
            (T_TABLESPACE, 10),
            (T_TABLE_CHECKSUM, 14),
            (T_TABLE_NAME, 10),
            (T_TEMPORARY, 9),
            (T_TEMPTABLE, 9),
            (T_TERMINATED, 10),
            (T_TEXT, 4),
            (T_TEXT_STRING, 11),
            (T_THAN, 4),
            (T_THEN, 4),
            (T_THREAD_PRIORITY, 15),
            (T_TIES, 4),
            (T_TIME, 4),
            (T_TIMESTAMP, 9),
            (T_TIMESTAMP_ADD, 13),
            (T_TIMESTAMP_DIFF, 14),
            (T_TINYBLOB, 8),
            (T_TINYINT, 7),
            (T_TINYTEXT_SYN, 12),
            (T_TLS, 3),
            (T_TRAILING, 8),
            (T_TRANSACTION, 11),
            (T_TRIGGER, 7),
            (T_TRIGGERS, 8),
            (T_TRIM, 4),
            (T_TRUE, 4),
            (T_TRUNCATE, 8),
            (T_TYPE, 4),
            (T_TYPES, 5),
            (T_ULONGLONG_NUM, 13),
            (T_UNBOUNDED, 9),
            (T_UNCOMMITTED, 11),
            (T_UNDEFINED, 9),
            (T_UNDERSCORE_CHARSET, 18),
            (T_UNDO, 4),
            (T_UNDOFILE, 8),
            (T_UNDO_BUFFER_SIZE, 16),
            (T_UNICODE, 7),
            (T_UNINSTALL, 9),
            (T_UNION, 5),
            (T_UNIQUE, 6),
            (T_UNKNOWN, 7),
            (T_UNLOCK, 6),
            (T_UNREGISTER, 10),
            (T_UNSIGNED, 8),
            (T_UNTIL, 5),
            (T_UPDATE, 6),
            (T_UPGRADE, 7),
            (T_USE, 3),
            (T_USER, 4),
            (T_USE_FRM, 7),
            (T_USING, 5),
            (T_UTC_DATE, 8),
            (T_UTC_TIME, 8),
            (T_UTC_TIMESTAMP, 13),
            (T_VALIDATION, 10),
            (T_VALUE, 5),
            (T_VALUES, 6),
            (T_VARBINARY, 9),
            (T_VARCHAR, 7),
            (T_VARIABLES, 9),
            (T_VARIANCE, 8),
            (T_VARYING, 7),
            (T_VAR_POP, 7),
            (T_VAR_SAMP, 8),
            (T_VCPU, 4),
            (T_VIEW, 4),
            (T_VISIBLE, 7),
            (T_WAIT, 4),
            (T_WARNINGS, 8),
            (T_WEEK, 4),
            (T_WEIGHT_STRING, 13),
            (T_WHEN, 4),
            (T_WHERE, 5),
            (T_WINDOW, 6),
            (T_WITH, 4),
            (T_WITHOUT, 7),
            (T_WORK, 4),
            (T_WRAPPER, 7),
            (T_X509, 4),
            (T_XA, 2),
            (T_XID, 3),
            (T_XML, 3),
            (T_XOR, 3),
            (T_XOR_OP, 1),
            (T_YEAR, 4),
            (T_YEAR_MONTH, 10),
            (T_ZEROFILL, 8),
            (T_ZONE, 4),
        ];
    };
}
