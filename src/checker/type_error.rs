use std::fmt::Display;

pub enum TypeError {
    DuplicateType(DuplicateType),
}

impl Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::DuplicateType(error) => write!(f, "{}", error),
        }
    }
}

pub struct DuplicateType {
    pub identifier: String,
}

impl Display for DuplicateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duplicate type `{}`", self.identifier)
    }
}
