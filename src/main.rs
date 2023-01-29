use sql_ddl_to_schema::{prompt_for_sql, convert_mysql_statement_to_property_list};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

fn main() {
    let sql = prompt_for_sql();

    let dialect = MySqlDialect {};

    let statements = Parser::parse_sql(&dialect, sql.as_str()).unwrap();
    let statement = statements.first().unwrap();

    let properties = convert_mysql_statement_to_property_list(statement);

    let yaml = serde_yaml::to_string(&properties).expect("Serialization error");
    println!("\n\n");
    println!("{}", yaml);
}
