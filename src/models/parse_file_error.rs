use std::fmt;

#[derive(Debug)]
pub struct ParseFileError(pub String);

impl fmt::Display for ParseFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse {} as file type", self.0)
    }
}
