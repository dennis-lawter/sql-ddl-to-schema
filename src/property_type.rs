use serde::Serialize;

#[derive(Debug, PartialEq, Eq)]
pub enum PropertyType {
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
    pub fn from_mysql_column_definition(input_str: &str) -> Result<Self, ()> {
        let upper_input = input_str.to_ascii_uppercase();

        if upper_input == "TINYINT(1)" {
            Ok(Self::Boolean)
        } else if upper_input.starts_with("INT") || upper_input.starts_with("BIGINT") || upper_input.starts_with("TINYINT") || upper_input.starts_with("SMALLINT") {
            Ok(Self::Integer)
        } else if upper_input.starts_with("DECIMAL") || upper_input.starts_with("FLOAT") {
            Ok(Self::Number)
        } else if upper_input.starts_with("VARCHAR") || upper_input.ends_with("TEXT") || upper_input.contains("DATE") || upper_input.contains("TIME") {
            Ok(Self::String)
        } else {
            Err(())
        }
    }
}
