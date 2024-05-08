use super::{EventId, NodeId};

pub struct Handle(pub u64);

impl Handle {
    pub fn new() -> Self {
        Self(rand::random())
    }
}

impl From<Handle> for NodeId {
    fn from(value: Handle) -> Self {
        NodeId(value.0)
    }
}

impl From<Handle> for EventId {
    fn from(value: Handle) -> Self {
        EventId(value.0)
    }
}
