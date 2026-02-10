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
    pub container_name: String,
    pub container_type: String,
    pub member_name: String,
    pub member_type: String,
}

impl Display for DuplicateMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Duplicate {} `{}` of {} `{}`",
            self.member_type, self.member_name, self.container_type, self.container_name
        )
    }
}
