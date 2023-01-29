use sql_ddl_to_schema::{convert_mysql_statement_to_property_list, property_item::PropertyItem, property_type::PropertyType};
use sqlparser::{dialect::MySqlDialect, parser::Parser};

#[test]
fn simple_case() {
    let sql = "\
    CREATE TABLE Persons (\
        PersonID int,\
        LastName varchar(255) not null,\
        FirstName varchar(255) null,\
		Registered tinyint(1) not null default 0,\
		Height decimal(10,2)\
    );";

    let dialect = MySqlDialect {};

	let statements_result = Parser::parse_sql(&dialect, sql);
	assert!(statements_result.is_ok());
	let statements = statements_result.unwrap();
	assert_eq!(1, statements.len());
	let statement_result = statements.first();
	assert!(statement_result.is_some());
	let statement = statement_result.unwrap();

	let properties = convert_mysql_statement_to_property_list(statement);

	assert!(properties.contains_key("PersonID"));
	let expected_person_id = PropertyItem { data_type: PropertyType::Integer, nullable: None };
	assert_eq!(expected_person_id, properties["PersonID"]);

	assert!(properties.contains_key("LastName"));
	let expected_person_id = PropertyItem { data_type: PropertyType::String, nullable: Some(false) };
	assert_eq!(expected_person_id, properties["LastName"]);

	assert!(properties.contains_key("FirstName"));
	let expected_person_id = PropertyItem { data_type: PropertyType::String, nullable: Some(true) };
	assert_eq!(expected_person_id, properties["FirstName"]);

	assert!(properties.contains_key("Registered"));
	let expected_person_id = PropertyItem { data_type: PropertyType::Boolean, nullable: Some(false) };
	assert_eq!(expected_person_id, properties["Registered"]);

	assert!(properties.contains_key("Height"));
	let expected_person_id = PropertyItem { data_type: PropertyType::Number, nullable: None };
	assert_eq!(expected_person_id, properties["Height"]);
}
