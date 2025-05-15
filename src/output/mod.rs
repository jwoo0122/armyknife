pub struct CommandResult<Value: std::fmt::Display> {
    pub status: bool,
    pub value: Value,
}
