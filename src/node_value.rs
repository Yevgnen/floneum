use floneum_plugin::exports::plugins::main::definitions::{Input, IoDefinition, Output};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NodeInput {
    pub definition: IoDefinition,
    pub value: Input,
}

impl NodeInput {
    pub fn new(definition: IoDefinition, value: Input) -> Self {
        Self { definition, value }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NodeOutput {
    pub definition: IoDefinition,
    pub value: Output,
}
