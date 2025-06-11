use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DataType {
    Integer(i32),
    Float(f64),
    String(String),
}

pub struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
    pub columns: HashMap<String, Vec<DataType>>,
    pub selected: Vec<String>,
}
