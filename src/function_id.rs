#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GetFunctionsArgument {
    Odbc2AllFunctions = 0,
    Odbc3AllFunctions = 999,
    Function(FunctionId),
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FunctionId {
    SqlAllocConnect = 1,
    SqlAllocEnv = 2,
    SqlAllocStmt = 3,
    SqlBindCol = 4,
    SqlCancel = 5,
    // SqlColAttributes used to have the same identifier (6) but has been replaced by SqlColAttribute in ODBC 3.x
    SqlColAttribute = 6,
    SqlConnect = 7,
    SqlDescribeCol = 8,
    SqlDisconnect = 9,
    SqlError = 10,
    SqlExecDirect = 11,
    SqlExecute = 12,
    SqlFetch = 13,
    SqlFreeConnect = 14,
    SqlFreeEnv = 15,
    SqlFreeStmt = 16,
    SqlGetCursorName = 17,
    SqlNumResultCols = 18,
    SqlPrepare = 19,
    SqlRowCount = 20,
    SqlSetCursorName = 21,
    SqlSetParam = 22,
    SqlTransact = 23,
    SqlBulkOperations = 24,

    SqlColumns = 40,
    SqlDriverConnect = 41,
    SqlGetConnectOption = 42,
    SqlGetData = 43,
    SqlGetFunctions = 44,
    SqlGetInfo = 45, // ODBC 1.0
    SqlGetStmtOption = 46,
    SqlGetTypeInfo = 47,
    SqlParameterData = 48,
    SqlPutData = 49,
    SqlSetConnectOption = 50,
    SqlSetStmtOption = 51,
    SqlSpecialColumns = 52,
    SqlStatistics = 53,
    SqlTables = 54,
    SqlBrowseConnect = 55,
    SqlColumnPrivileges = 56,
    SqlDataSources = 57,
    SqlDescribeParam = 58,
    SqlExtendedFetch = 59,
    SqlForeignKeys = 60,
    SqlMoreResults = 61,
    SqlNativeSql = 62,
    SqlNumParams = 63,
    SqlParamOptions = 64,
    SqlPrimaryKeys = 65,
    SqlProcedureColumns = 66,
    SqlProcedures = 67,
    SqlSetPos = 68,
    SqlSetScrollOptions = 69,
    SqlTablePrivileges = 70,
    SqlDrivers = 71,
    SqlBindParameter = 72,
    SqlAllocHandleStd = 73,
    SqlGetNestedHandle = 74,
    SqlStructuredTypes = 75,
    SqlStructuredTypeColumns = 76,
    SqlNextColumn = 77,

    SqlAllocHandle = 1001,
    SqlBindParam = 1002,
    SqlCloseCursor = 1003,
    SqlCopyDesc = 1004,
    SqlEndTran = 1005,
    SqlFreeHandle = 1006,
    SqlGetConnectAttr = 1007,
    SqlGetDescField = 1008,
    SqlGetDescRec = 1009,
    SqlGetDiagField = 1010,
    SqlGetDiagRec = 1011,
    SqlGetEnvAttr = 1012,
    SqlGetStmtAttr = 1014,
    SqlSetConnectAttr = 1016,
    SqlSetDescField = 1017,
    SqlSetDescRec = 1018,
    SqlSetEnvAttr = 1019,
    SqlSetStmtAttr = 1020,

    SqlFetchScroll = 1021,
    SqlCancelHandle = 1550,
    SqlCompleteAsync = 1551,
}

/*

#define SQL_API_SQLALLOCCONNECT         1
#define SQL_API_SQLALLOCENV             2
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLALLOCHANDLE          1001
#endif
#define SQL_API_SQLALLOCSTMT            3
#define SQL_API_SQLBINDCOL              4
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLBINDPARAM            1002
#endif
#define SQL_API_SQLCANCEL               5
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLCLOSECURSOR          1003
#define SQL_API_SQLCOLATTRIBUTE         6
#endif
#define SQL_API_SQLCOLUMNS              40
#define SQL_API_SQLCONNECT              7
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLCOPYDESC             1004
#endif
#define SQL_API_SQLDATASOURCES          57
#define SQL_API_SQLDESCRIBECOL          8
#define SQL_API_SQLDISCONNECT           9
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLENDTRAN              1005
#endif
#define SQL_API_SQLERROR                10
#define SQL_API_SQLEXECDIRECT           11
#define SQL_API_SQLEXECUTE              12
#define SQL_API_SQLFETCH                13
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLFETCHSCROLL          1021
#endif
#define SQL_API_SQLFREECONNECT          14
#define SQL_API_SQLFREEENV              15
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLFREEHANDLE           1006
#endif
#define SQL_API_SQLFREESTMT             16
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLGETCONNECTATTR       1007
#endif
#define SQL_API_SQLGETCONNECTOPTION     42
#define SQL_API_SQLGETCURSORNAME        17
#define SQL_API_SQLGETDATA              43
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLGETDESCFIELD         1008
#define SQL_API_SQLGETDESCREC           1009
#define SQL_API_SQLGETDIAGFIELD         1010
#define SQL_API_SQLGETDIAGREC           1011
#define SQL_API_SQLGETENVATTR           1012
#endif
#define SQL_API_SQLGETFUNCTIONS         44
#define SQL_API_SQLGETINFO              45
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLGETSTMTATTR          1014
#endif
#define SQL_API_SQLGETSTMTOPTION        46
#define SQL_API_SQLGETTYPEINFO          47
#define SQL_API_SQLNUMRESULTCOLS        18
#define SQL_API_SQLPARAMDATA            48
#define SQL_API_SQLPREPARE              19
#define SQL_API_SQLPUTDATA              49
#define SQL_API_SQLROWCOUNT             20
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLSETCONNECTATTR       1016
#endif
#define SQL_API_SQLSETCONNECTOPTION     50
#define SQL_API_SQLSETCURSORNAME        21
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLSETDESCFIELD         1017
#define SQL_API_SQLSETDESCREC           1018
#define SQL_API_SQLSETENVATTR           1019
#endif
#define SQL_API_SQLSETPARAM             22
#if (ODBCVER >= 0x0300)
#define SQL_API_SQLSETSTMTATTR          1020
#endif
#define SQL_API_SQLSETSTMTOPTION        51
#define SQL_API_SQLSPECIALCOLUMNS       52
#define SQL_API_SQLSTATISTICS           53
#define SQL_API_SQLTABLES               54
#define SQL_API_SQLTRANSACT             23
#if (ODBCVER >= 0x0380)
#define SQL_API_SQLCANCELHANDLE         1550
#define SQL_API_SQLCOMPLETEASYNC        1551



/********************************************/
/* SQLGetFunctions: additional values for   */
/* fFunction to represent functions that    */
/* are not in the X/Open spec.              */
/********************************************/

#if (ODBCVER >= 0x0300)
#define SQL_API_SQLALLOCHANDLESTD   73
#define SQL_API_SQLBULKOPERATIONS   24
#endif /* ODBCVER >= 0x0300 */
#define SQL_API_SQLBINDPARAMETER    72
#define SQL_API_SQLBROWSECONNECT    55
#define SQL_API_SQLCOLATTRIBUTES    6
#define SQL_API_SQLCOLUMNPRIVILEGES 56
#define SQL_API_SQLDESCRIBEPARAM    58
#define SQL_API_SQLDRIVERCONNECT    41
#define SQL_API_SQLDRIVERS          71
#define SQL_API_SQLEXTENDEDFETCH    59
#define SQL_API_SQLFOREIGNKEYS      60
#define SQL_API_SQLMORERESULTS      61
#define SQL_API_SQLNATIVESQL        62
#define SQL_API_SQLNUMPARAMS        63
#define SQL_API_SQLPARAMOPTIONS     64
#define SQL_API_SQLPRIMARYKEYS      65
#define SQL_API_SQLPROCEDURECOLUMNS 66
#define SQL_API_SQLPROCEDURES       67
#define SQL_API_SQLSETPOS           68
#define SQL_API_SQLSETSCROLLOPTIONS 69
#define SQL_API_SQLTABLEPRIVILEGES  70

#if (ODBCVER >= 0x0400)
#define SQL_API_SQLGETNESTEDHANDLE       74
#define SQL_API_SQLSTRUCTUREDTYPES       75
#define SQL_API_SQLSTRUCTUREDTYPECOLUMNS 76
#define SQL_API_SQLNEXTCOLUMN            77
#endif /* ODBCVER >= 0x0400 */

/*-------------------------------------------*/
/* SQL_EXT_API_LAST is not useful with ODBC  */
/* version 3.0 because some of the values    */
/* from X/Open are in the 10000 range.       */
/*-------------------------------------------*/

#if (ODBCVER < 0x0300)
#define SQL_EXT_API_LAST            SQL_API_SQLBINDPARAMETER
#define SQL_NUM_FUNCTIONS           23
#define SQL_EXT_API_START           40
#define SQL_NUM_EXTENSIONS (SQL_EXT_API_LAST-SQL_EXT_API_START+1)
#endif

/*--------------------------------------------*/
/* SQL_API_ALL_FUNCTIONS returns an array     */
/* of 'booleans' representing whether a       */
/* function is implemented by the driver.     */
/*                                            */
/* CAUTION: Only functions defined in ODBC    */
/* version 2.0 and earlier are returned, the  */
/* new high-range function numbers defined by */
/* X/Open break this scheme.   See the new    */
/* method -- SQL_API_ODBC3_ALL_FUNCTIONS      */
/*--------------------------------------------*/

#define SQL_API_ALL_FUNCTIONS       0       /* See CAUTION above */

/*----------------------------------------------*/
/* 2.X drivers export a dummy function with     */
/* ordinal number SQL_API_LOADBYORDINAL to speed*/
/* loading under the windows operating system.  */
/*                      */
/* CAUTION: Loading by ordinal is not supported */
/* for 3.0 and above drivers.           */
/*----------------------------------------------*/

#define SQL_API_LOADBYORDINAL       199     /* See CAUTION above */

/*----------------------------------------------*/
/* SQL_API_ODBC3_ALL_FUNCTIONS                  */
/* This returns a bitmap, which allows us to    */
/* handle the higher-valued function numbers.   */
/* Use  SQL_FUNC_EXISTS(bitmap,function_number) */
/* to determine if the function exists.         */
/*----------------------------------------------*/


#if (ODBCVER >= 0x0300)
#define SQL_API_ODBC3_ALL_FUNCTIONS 999
#define SQL_API_ODBC3_ALL_FUNCTIONS_SIZE    250     /* array of 250 words */


 */
