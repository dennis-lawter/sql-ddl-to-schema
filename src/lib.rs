pub mod property_type;
pub mod property_item;

use linear_map::LinearMap;
use property_item::PropertyItem;
use property_type::PropertyType;
use sqlparser::ast::{ColumnDef, Statement};

pub fn prompt_for_sql() -> String {
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

pub fn is_column_nullable(column: &ColumnDef) -> Option<bool> {
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

pub fn convert_mysql_statement_to_property_list(statement: &Statement) -> LinearMap<String, PropertyItem> {
    let mut properties: LinearMap<String, PropertyItem> = LinearMap::new();

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
                        properties.insert(name.clone(), PropertyItem { data_type, nullable });
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

    properties
}

