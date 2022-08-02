pub struct TableColumn {
    pub column_name: String,
    pub data_type: String,
    pub character_maximum_length: i32,
    pub column_default: String,
    pub is_nullable: bool,
    pub is_identity: bool,
    pub table_name: String,
}

// pub fn new() -> Self {
// 	Self{}
// }
