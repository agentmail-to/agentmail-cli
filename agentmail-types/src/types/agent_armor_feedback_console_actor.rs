pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentArmorFeedbackConsoleActor {
}

impl AgentArmorFeedbackConsoleActor {
    pub fn builder() -> AgentArmorFeedbackConsoleActorBuilder {
        <AgentArmorFeedbackConsoleActorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentArmorFeedbackConsoleActorBuilder {
}

impl AgentArmorFeedbackConsoleActorBuilder {

    /// Consumes the builder and constructs a [`AgentArmorFeedbackConsoleActor`].
    pub fn build(self) -> Result<AgentArmorFeedbackConsoleActor, BuildError> {
        Ok(AgentArmorFeedbackConsoleActor {
        })
    }
}
