use {
    crate::result::{Error, Result},
    sqlparser::{
        ast::{
            Assignment as SqlAssignment, ColumnDef as SqlColumnDef, DataType as SqlDataType,
            Expr as SqlExpr, Ident as SqlIdent, OrderByExpr as SqlOrderByExpr, Query as SqlQuery,
            SelectItem as SqlSelectItem, Statement as SqlStatement,
        },
        dialect::GenericDialect,
        parser::Parser,
        tokenizer::Tokenizer,
    },
};

const DIALECT: GenericDialect = GenericDialect {};

pub fn parse<Sql: AsRef<str>>(sql: Sql) -> Result<Vec<SqlStatement>> {
    Parser::parse_sql(&DIALECT, sql.as_ref()).map_err(|e| Error::Parser(format!("{:#?}", e)))
}

macro_rules! impl_parse_method {
    ($method_name: ident, $return_type: ty) => {
        pub fn $method_name<Sql: AsRef<str>>(sql: Sql) -> Result<$return_type> {
            let tokens = Tokenizer::new(&DIALECT, sql.as_ref())
                .tokenize()
                .map_err(|e| Error::Parser(format!("{:#?}", e)))?;

            Parser::new(tokens, &DIALECT)
                .$method_name()
                .map_err(|e| Error::Parser(format!("{:#?}", e)))
        }
    };

    ($method_name: ident, $parser_method: ident, $return_type: ty) => {
        pub fn $method_name<Sql: AsRef<str>>(sql: Sql) -> Result<$return_type> {
            let tokens = Tokenizer::new(&DIALECT, sql.as_ref())
                .tokenize()
                .map_err(|e| Error::Parser(format!("{:#?}", e)))?;

            Parser::new(tokens, &DIALECT)
                .$parser_method()
                .map_err(|e| Error::Parser(format!("{:#?}", e)))
        }
    };

    ($method_name: ident, $parser_method: ident, $parser_arg: path, $return_type: ty) => {
        pub fn $method_name<Sql: AsRef<str>>(sql: Sql) -> Result<$return_type> {
            let tokens = Tokenizer::new(&DIALECT, sql.as_ref())
                .tokenize()
                .map_err(|e| Error::Parser(format!("{:#?}", e)))?;

            Parser::new(tokens, &DIALECT)
                .$parser_method($parser_arg)
                .map_err(|e| Error::Parser(format!("{:#?}", e)))
        }
    };
}

impl_parse_method!(parse_query, SqlQuery);
impl_parse_method!(parse_expr, SqlExpr);
impl_parse_method!(parse_select_item, SqlSelectItem);
impl_parse_method!(parse_order_by_expr, SqlOrderByExpr);
impl_parse_method!(parse_column_def, SqlColumnDef);
impl_parse_method!(parse_data_type, SqlDataType);
impl_parse_method!(parse_assignment, SqlAssignment);
impl_parse_method!(parse_identifiers, Vec<SqlIdent>);

impl_parse_method!(parse_interval, SqlExpr);

impl_parse_method!(
    parse_comma_separated_exprs,
    parse_comma_separated,
    Parser::parse_expr,
    Vec<SqlExpr>
);
impl_parse_method!(
    parse_select_items,
    parse_comma_separated,
    Parser::parse_select_item,
    Vec<SqlSelectItem>
);
impl_parse_method!(
    parse_order_by_exprs,
    parse_comma_separated,
    Parser::parse_order_by_expr,
    Vec<SqlOrderByExpr>
);
