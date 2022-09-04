// https://github.com/serenity-rs/serenity/blob/current/examples/e17_message_components/src/main.rs
use crate::entity::sea_orm_active_enums::RestrictionType;
use crate::models::parse_component_error::ParseComponentError;
use poise::serenity_prelude::{CreateActionRow, CreateSelectMenu, CreateSelectMenuOption};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

impl fmt::Display for RestrictionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attachment => write!(f, "Attachment"),
            Self::Image => write!(f, "Image"),
            Self::None => write!(f, "None"),
            Self::Text => write!(f, "Text"),
            Self::Video => write!(f, "Video"),
        }
    }
}

impl FromStr for RestrictionType {
    type Err = ParseComponentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "attachment" => Ok(RestrictionType::Attachment),
            "image" => Ok(RestrictionType::Image),
            "none" => Ok(RestrictionType::None),
            "text" => Ok(RestrictionType::Text),
            "video" => Ok(RestrictionType::Video),
            _ => Err(ParseComponentError(s.to_string())),
        }
    }
}

impl RestrictionType {
    pub fn get_members() -> Vec<String> {
        let members = vec![
            RestrictionType::Attachment,
            RestrictionType::Image,
            RestrictionType::Video,
            RestrictionType::None,
            RestrictionType::Text,
        ];

        members.iter().map(|rt| rt.to_string()).collect()
    }

    pub fn emoji(&self) -> char {
        match self {
            Self::Attachment => '📁',
            Self::Image => '📷',
            Self::None => '🟢',
            Self::Text => '📖',
            Self::Video => '📹',
        }
    }

    pub fn menu_option(&self) -> CreateSelectMenuOption {
        let mut opt = CreateSelectMenuOption::default();
        opt.label(format!("{} {}", self.emoji(), self.to_string().to_uppercase()));
        opt.value(self.to_string().to_ascii_lowercase());
        opt
    }

    pub fn select_menu() -> CreateSelectMenu {
        let mut menu = CreateSelectMenu::default();
        menu.custom_id("restriction_select");
        menu.placeholder("No restriction selected");
        menu.options(|f| {
            f.add_option(Self::Attachment.menu_option())
                .add_option(Self::Image.menu_option())
                .add_option(Self::Video.menu_option())
                .add_option(Self::Text.menu_option())
                .add_option(Self::None.menu_option())
        });

        menu
    }

    pub fn action_row() -> CreateActionRow {
        let mut ar = CreateActionRow::default();

        ar.add_select_menu(Self::select_menu());
        ar
    }
}
