use crate::entity::sea_orm_active_enums::RestrictionType;
use crate::models::parse_component_error::ParseComponentError;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

impl fmt::Display for RestrictionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attachment => write!(f, "attachment"),
            Self::Image => write!(f, "image"),
            Self::None => write!(f, "none"),
            Self::Text => write!(f, "text"),
            Self::Video => write!(f, "video"),
        }
    }
}

impl FromStr for RestrictionType {
    type Err = ParseComponentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "attachment" => Ok(RestrictionType::Attachment),
            "image" => Ok(RestrictionType::Image),
            "none" => Ok(RestrictionType::None),
            "text" => Ok(RestrictionType::Text),
            "video" => Ok(RestrictionType::Video),
            _ => Err(ParseComponentError(s.to_string())),
        }
    }
}

impl RestrictionType {}
