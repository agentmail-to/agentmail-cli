pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Always `PUT`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FileUploadMethod {
    #[serde(rename = "PUT")]
    Put,
}
impl fmt::Display for FileUploadMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Put => "PUT",
        };
        write!(f, "{}", s)
    }
}
