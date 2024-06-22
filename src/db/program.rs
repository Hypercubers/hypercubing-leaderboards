pub struct Program {
    pub id: i32,
    pub name: String,
    pub abbreviation: String,
}

pub struct ProgramVersion {
    pub id: i32,
    pub program: Program,
    pub version: Option<String>,
}
