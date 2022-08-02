pub struct TableColumn {
    pub column_name: String,
    pub data_type: String,
    pub character_maximum_length: i32,
    pub column_default: String,
    pub is_nullable: bool,
    pub is_identity: i32,
    pub table_name: String,
    pub constraint_name: String,
    pub numeric_precision: u8,
    pub numeric_scale: i32,
}

// pub fn new() -> Self {
// 	Self{}
// }
