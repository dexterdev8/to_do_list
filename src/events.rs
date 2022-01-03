/*
Ref: https://docs.near.org/docs/tutorials/contracts/nfts/events
*/

/*
Equivalent in Solidity

event TaskCreated(
    uint id,
    string content,
    bool completed
);

event TaskCompleted(
    uint id,
    bool completed
);
*/
use std::fmt;

use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    CreateTask(Vec<CreateTaskLog>),
    ToggleCompleted(Vec<ToggleCompletedLog>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateTaskLog {
    pub id: u8,
    pub content: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ToggleCompletedLog {
    pub id: u8,
    pub completed: bool,
}
