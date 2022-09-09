use crate::entity::sea_orm_active_enums::FileType;
use crate::models::parse_file_error::ParseFileError;
use std::str::FromStr;

impl FromStr for FileType {
    type Err = ParseFileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        let file_split: Vec<&str> = lower.as_str().split('.').collect();
        match file_split[file_split.len() - 1] {
            "jpg" | "jpeg" | "gif" | "png" | "bmp" | "tiff" => Ok(FileType::Image),
            "mp4" | "mkv" | "mov" | "avi" | "flv" | "webm" | "wmv" => Ok(FileType::Video),
            _ => Ok(FileType::Other),
        }
    }
}
