use super::{EventId, NodeId};

pub struct Handle(u64);

impl Handle {
    pub fn new(id: u64) -> Self {
        Self(id)
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
