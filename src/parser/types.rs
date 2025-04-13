pub mod expected {
    pub const EXPRESSION: &str = "expression";
    pub const STATEMENT: &str = "statement";
    pub const IDENTIFIER: &str = "identifier";
    pub const SEMICOLON: &str = "semicolon";
    pub const COLON: &str = ":";
    pub const LEFT_PAREN: &str = "(";
    pub const RIGHT_PAREN: &str = ")";
    pub const LEFT_BRACE: &str = "{";
    pub const RIGHT_BRACE: &str = "}";
    pub const LEFT_BRACKET: &str = "[";
    pub const RIGHT_BRACKET: &str = "]";
    pub const VARIABLE_TYPE: &str = "variable type";
    pub const CLASS_NAME: &str = "class name";
    pub const METHOD_NAME: &str = "method name";
    pub const PARAMETER_NAME: &str = "parameter name";
    pub const RETURN_TYPE: &str = "return type";
}

pub enum SyncPoint {
    ClassBody,
    MethodBody,
    Statement,
    Expression,
}
