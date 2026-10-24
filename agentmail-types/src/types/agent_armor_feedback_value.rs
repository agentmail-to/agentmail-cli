pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Your assessment of a completed verdict.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentArmorFeedbackValue {
    CorrectReview,
    IncorrectReview,
    MissedThreat,
    LegitimateAgentInstruction,
    SpamOrPhishing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentArmorFeedbackValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CorrectReview => serializer.serialize_str("correct_review"),
            Self::IncorrectReview => serializer.serialize_str("incorrect_review"),
            Self::MissedThreat => serializer.serialize_str("missed_threat"),
            Self::LegitimateAgentInstruction => serializer.serialize_str("legitimate_agent_instruction"),
            Self::SpamOrPhishing => serializer.serialize_str("spam_or_phishing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentArmorFeedbackValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "correct_review" => Ok(Self::CorrectReview),
            "incorrect_review" => Ok(Self::IncorrectReview),
            "missed_threat" => Ok(Self::MissedThreat),
            "legitimate_agent_instruction" => Ok(Self::LegitimateAgentInstruction),
            "spam_or_phishing" => Ok(Self::SpamOrPhishing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentArmorFeedbackValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CorrectReview => write!(f, "correct_review"),
            Self::IncorrectReview => write!(f, "incorrect_review"),
            Self::MissedThreat => write!(f, "missed_threat"),
            Self::LegitimateAgentInstruction => write!(f, "legitimate_agent_instruction"),
            Self::SpamOrPhishing => write!(f, "spam_or_phishing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
