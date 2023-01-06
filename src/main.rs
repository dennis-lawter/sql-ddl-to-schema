use linear_map::LinearMap;
use serde::Serialize;
use sqlparser::ast::{ColumnDef, Statement};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

#[derive(Debug)]
enum PropertyType {
    String,
    Number,
    Integer,
    Boolean,
    // Array,
    // Object,
}
impl ToString for PropertyType {
    fn to_string(&self) -> String {
        match self {
            PropertyType::String => String::from("string"),
            PropertyType::Number => String::from("number"),
            PropertyType::Integer => String::from("integer"),
            PropertyType::Boolean => String::from("boolean"),
            // PropertyType::Array => String::from("array"),
            // PropertyType::Object => String::from("object"),
        }
    }
}
impl Serialize for PropertyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
impl PropertyType {
    fn from_mysql_column_definition(input_str: &str) -> Result<Self, ()> {
        let upper_input = input_str.to_ascii_uppercase();
        if upper_input == "TINYINT(1)" {
            return Ok(Self::Boolean);
        } else if upper_input.starts_with("INT") {
            return Ok(Self::Integer);
        } else if upper_input.starts_with("VARCHAR") || upper_input.ends_with("TEXT") {
            return Ok(Self::String);
        } else if upper_input.starts_with("DECIMAL") || upper_input.starts_with("FLOAT") {
            return Ok(Self::Number);
        }

        Err(())
    }
}

#[derive(Debug, Serialize)]
struct PropertyItem {
    // name: String,
    #[serde(rename(serialize = "type"))]
    data_type: PropertyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<bool>,
}

fn is_column_nullable(column: &ColumnDef) -> Option<bool> {
    for option in column.options.iter() {
        let test = &option.option;
        match test {
            sqlparser::ast::ColumnOption::Null => return Some(true),
            sqlparser::ast::ColumnOption::NotNull => return Some(false),
            _ => {}
        }
    }

    None
}

fn prompt_for_sql() -> String {
    println!();
    let prompt = String::from("Enter SQL statement (ending in semicolon):");
    println!("{}", prompt);
    let mut input_statement = String::new();
    while !input_statement.contains(";") {
        let mut buffer = String::new();
        std::io::Stdin::read_line(&std::io::stdin(), &mut buffer).unwrap();
        input_statement.push_str(buffer.as_str());
    }
    input_statement.trim().to_owned()
}

fn main() {
    let sql = prompt_for_sql();

    let dialect = MySqlDialect {};

    let ast = Parser::parse_sql(&dialect, sql.as_str()).unwrap();

    let mut props: LinearMap<String, PropertyItem> = LinearMap::new();

    let statement = &ast[0];
    match statement {
        Statement::CreateTable {
            or_replace: _,
            temporary: _,
            external: _,
            global: _,
            if_not_exists: _,
            name: _,
            columns,
            constraints: _,
            hive_distribution: _,
            hive_formats: _,
            table_properties: _,
            with_options: _,
            file_format: _,
            location: _,
            query: _,
            without_rowid: _,
            like: _,
            clone: _,
            engine: _,
            default_charset: _,
            collation: _,
            on_commit: _,
            on_cluster: _,
        } => {
            for column in columns {
                let name = &column.name.value;
                let data_type_string = &column.data_type.to_string();
                let nullable = is_column_nullable(column);
                // println!("{:?}", column);
                let data_type_result =
                    PropertyType::from_mysql_column_definition(data_type_string.as_str());
                match data_type_result {
                    Ok(data_type) => {
                        props.insert(name.clone(), PropertyItem { data_type, nullable });
                    }
                    Err(_) => {
                        panic!("Unsupported column: {}", data_type_string);
                    }
                }
            }
        }
        _ => {
            panic!("Unsupported SQL statement.");
        }
    }

    let yaml = serde_yaml::to_string(&props).expect("Serialization error");
    println!("{}", yaml);
}
