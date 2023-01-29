use serde::Serialize;

use crate::property_type::PropertyType;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct PropertyItem {
    #[serde(rename(serialize = "type"))]
    pub data_type: PropertyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
}
