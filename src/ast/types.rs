use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeName {
    Int,
    Str,
    Boolean,
    Void,
    Class(String),
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeName::Int => write!(f, "Int"),
            TypeName::Str => write!(f, "Str"),
            TypeName::Boolean => write!(f, "Boolean"),
            TypeName::Void => write!(f, "Void"),
            TypeName::Class(name) => write!(f, "{}", name),
        }
    }
}
