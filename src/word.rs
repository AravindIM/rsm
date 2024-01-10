#[derive(Clone)]
pub enum Word {
    Undefined,
    String { value: String },
    Integer { value: i32 },
}
