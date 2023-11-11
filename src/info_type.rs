use crate::{UInteger, USmallInt};
use num_enum::TryFromPrimitive;

pub enum InfoTypeTypeInformation {
    BoolString,
    String,
    SqlUSmallInt,
    SqlUInteger,
}

impl InfoTypeTypeInformation {
    pub fn not_supported_value(&self) -> InfoTypeType {
        match self {
            InfoTypeTypeInformation::BoolString => InfoTypeType::String("N".to_string()),
            InfoTypeTypeInformation::String => InfoTypeType::String("".to_string()),
            InfoTypeTypeInformation::SqlUSmallInt => InfoTypeType::SqlUSmallInt(0),
            InfoTypeTypeInformation::SqlUInteger => InfoTypeType::SqlUInteger(0),
        }
    }
}

pub enum InfoTypeType {
    String(String),
    SqlUSmallInt(USmallInt),
    SqlUInteger(UInteger),
}

impl InfoTypeType {
    pub fn len(&self) -> usize {
        match self {
            InfoTypeType::String(string) => {
                /*
                let c_string = CString::new(string).unwrap();
                let c_string_bytes = c_string.as_bytes();
                c_string_bytes.len()

                 */
                string.len()
            }
            InfoTypeType::SqlUSmallInt(_) => 2,
            InfoTypeType::SqlUInteger(_) => 4,
        }
    }
}

/// Information requested by SQLGetInfo
#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
pub enum InfoType {
    MaxDriverConnections = 0,
    MaxConcurrentActivities = 1,
    DataSourceName = 2,
    // FetchDirection = 8, Deprecated in ODBC 3
    ServerName = 13,
    SearchPatternEscape = 14,
    DbmsName = 17,
    DbmsVer = 18,
    AccessibleTables = 19,
    AccessibleProcedures = 20,
    CursorCommitBehaviour = 23,
    DataSourceReadOnly = 25,
    DefaultTxnIsolation = 26,
    IdentifierCase = 28,
    IdentifierQuoteChar = 29,
    MaxColumnNameLen = 30,
    MaxCursorNameLen = 31,
    MaxSchemaNameLen = 32,
    MaxCatalogNameLen = 34,
    MaxTableNameLen = 35,
    // ScrollConcurrency = 43, deprecated in ODBC 3
    /// `SQL_SCROLL_OPTIONS` C-API places this in the extended header. Lists the supported cursor
    /// types (forward-only, static, keyset-driven, dynamic, or mixed). All data sources must
    /// support forward-only cursors. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    ScrollOptions = 44,
    TxnCapable = 46,
    UserName = 47,
    TxnIsolationOption = 72,
    Integrity = 73,
    GetDataExtensions = 81,
    NullCollation = 85,
    AlterTable = 86,
    OrderByColumnsInSelect = 90,
    SpecialCharacters = 94,
    MaxColumnsInGroupBy = 97,
    MaxColumnsInIndex = 98,
    MaxColumnsInOrderBy = 99,
    MaxColumnsInSelect = 100,
    MaxColumnsInTable = 101,
    MaxIndexSize = 102,
    MaxRowSize = 104,
    MaxStatementLen = 105,
    MaxTablesInSelect = 106,
    MaxUserNameLen = 107,
    OuterJoinCapabilities = 115,
    /// `SQL_ACTIVE_ENVIRONMENTS` C-API places this in the extended header. A `u16` value that
    /// specifies the maximum number of active environments that the driver can support. If there is
    /// no specified limit or the limit is unknown, this value is set to zero.
    ActiveEnvironments = 116,
    /// `SQL_DYNAMIC_CURSOR_ATTRIBUTES1` C-API places this in the extended header. Lists the fetch
    /// types supported by scrollable cursors. The bits in the return value correspond to the fetch
    /// types in SQLFetchScroll. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    DynamicCursorAttributes1 = 144,
    /// `SQL_DYNAMIC_CURSOR_ATTRIBUTES2` C-API places this in the extended header. Lists whether
    /// cursors can detect their own updates, deletes, and inserts. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    DynamicCursorAttributes2 = 145,
    /// `SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1` C-API places this in the extended header. Lists the
    /// fetch types supported by scrollable cursors. The bits in the return value correspond to the
    /// fetch types in SQLFetchScroll. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    ForwardOnlyCursorAttributes1 = 146,
    /// `SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2` C-API places this in the extended header. Lists
    /// whether cursors can detect their own updates, deletes, and inserts. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    ForwardOnlyCursorAttributes2 = 147,
    /// `SQL_KEYSET_CURSOR_ATTRIBUTES1` C-API places this in the extended header. Lists the fetch
    /// types supported by scrollable cursors. The bits in the return value correspond to the fetch
    /// types in SQLFetchScroll. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    KeysetCursorAttributes1 = 150,
    /// `SQL_KEYSET_CURSOR_ATTRIBUTES2` C-API places this in the extended header. Lists whether
    /// cursors can detect their own updates, deletes, and inserts. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    KeysetCursorAttributes2 = 151,
    /// `SQL_STATIC_CURSOR_ATTRIBUTES1` C-API places this in the extended header. Lists the fetch
    /// types supported by scrollable cursors. The bits in the return value correspond to the fetch
    /// types in SQLFetchScroll. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    StaticCursorAttributes1 = 167,
    /// `SQL_STATIC_CURSOR_ATTRIBUTES2` C-API places this in the extended header. Lists whether
    /// cursors can detect their own updates, deletes, and inserts. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    StaticCursorAttributes2 = 168,
    XopenCliYear = 10000,
    CursorSensitivity = 10001,
    DescribeParameter = 10002,
    CatalogName = 10003,
    CollationSeq = 10004,
    MaxIdentifierLen = 10005,
    AsyncMode = 10021,
    MaxAsyncConcurrentStatements = 10022,
    AsyncDbcFunctions = 10023,
    DriverAwarePoolingSupported = 10024,
    AsyncNotification = 10025,
}

impl InfoType {
    pub fn return_type(&self) -> InfoTypeTypeInformation {
        match self {
            InfoType::MaxDriverConnections => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxConcurrentActivities => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::DataSourceName => InfoTypeTypeInformation::String,
            InfoType::ServerName => InfoTypeTypeInformation::String,
            InfoType::SearchPatternEscape => InfoTypeTypeInformation::String,
            InfoType::DbmsName => InfoTypeTypeInformation::String,
            InfoType::DbmsVer => InfoTypeTypeInformation::String,
            InfoType::AccessibleTables => InfoTypeTypeInformation::BoolString,
            InfoType::AccessibleProcedures => InfoTypeTypeInformation::BoolString,
            InfoType::CursorCommitBehaviour => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::DataSourceReadOnly => InfoTypeTypeInformation::BoolString,
            InfoType::DefaultTxnIsolation => InfoTypeTypeInformation::SqlUInteger,
            InfoType::IdentifierCase => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::IdentifierQuoteChar => InfoTypeTypeInformation::String,
            InfoType::MaxColumnNameLen => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxCursorNameLen => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxSchemaNameLen => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxCatalogNameLen => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxTableNameLen => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::ScrollOptions => InfoTypeTypeInformation::SqlUInteger,
            InfoType::TxnCapable => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::UserName => InfoTypeTypeInformation::String,
            InfoType::TxnIsolationOption => InfoTypeTypeInformation::SqlUInteger,
            InfoType::Integrity => InfoTypeTypeInformation::BoolString,
            InfoType::GetDataExtensions => InfoTypeTypeInformation::SqlUInteger,
            InfoType::NullCollation => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::AlterTable => InfoTypeTypeInformation::SqlUInteger,
            InfoType::OrderByColumnsInSelect => InfoTypeTypeInformation::BoolString,
            InfoType::SpecialCharacters => InfoTypeTypeInformation::String,
            InfoType::MaxColumnsInGroupBy => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxColumnsInIndex => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxColumnsInOrderBy => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxColumnsInSelect => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxColumnsInTable => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxIndexSize => InfoTypeTypeInformation::SqlUInteger,
            InfoType::MaxRowSize => InfoTypeTypeInformation::SqlUInteger,
            InfoType::MaxStatementLen => InfoTypeTypeInformation::SqlUInteger,
            InfoType::MaxTablesInSelect => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::MaxUserNameLen => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::OuterJoinCapabilities => InfoTypeTypeInformation::SqlUInteger,
            InfoType::ActiveEnvironments => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::DynamicCursorAttributes1 => InfoTypeTypeInformation::SqlUInteger,
            InfoType::DynamicCursorAttributes2 => InfoTypeTypeInformation::SqlUInteger,
            InfoType::ForwardOnlyCursorAttributes1 => InfoTypeTypeInformation::SqlUInteger,
            InfoType::ForwardOnlyCursorAttributes2 => InfoTypeTypeInformation::SqlUInteger,
            InfoType::KeysetCursorAttributes1 => InfoTypeTypeInformation::SqlUInteger,
            InfoType::KeysetCursorAttributes2 => InfoTypeTypeInformation::SqlUInteger,
            InfoType::StaticCursorAttributes1 => InfoTypeTypeInformation::SqlUInteger,
            InfoType::StaticCursorAttributes2 => InfoTypeTypeInformation::SqlUInteger,
            InfoType::XopenCliYear => InfoTypeTypeInformation::String,
            InfoType::CursorSensitivity => InfoTypeTypeInformation::SqlUInteger,
            InfoType::DescribeParameter => InfoTypeTypeInformation::BoolString,
            InfoType::CatalogName => InfoTypeTypeInformation::BoolString,
            InfoType::CollationSeq => InfoTypeTypeInformation::String,
            InfoType::MaxIdentifierLen => InfoTypeTypeInformation::SqlUSmallInt,
            InfoType::AsyncMode => InfoTypeTypeInformation::SqlUInteger,
            InfoType::MaxAsyncConcurrentStatements => InfoTypeTypeInformation::SqlUInteger,
            InfoType::AsyncDbcFunctions => InfoTypeTypeInformation::SqlUInteger,
            InfoType::DriverAwarePoolingSupported => InfoTypeTypeInformation::SqlUInteger,
            InfoType::AsyncNotification => InfoTypeTypeInformation::SqlUInteger,
        }
    }
}
