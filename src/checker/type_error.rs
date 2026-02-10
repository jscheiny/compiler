use std::fmt::Display;

pub enum TypeError {
    DuplicateMemberName(DuplicateMemberName),
    DuplicateType(DuplicateType),
}

impl Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::DuplicateMemberName(error) => write!(f, "{}", error),
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

pub struct DuplicateMemberName {
    pub member_name: String,
    pub container_name: String,
    pub container_type: String, // TODO change to enum
}

impl Display for DuplicateMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Duplicate member `{}` of {} `{}`",
            self.member_name, self.container_type, self.container_name
        )
    }
}
