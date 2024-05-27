use super::{EventId, NodeId};

pub struct IdOffset(pub u64);

impl IdOffset {
    pub fn new() -> Self {
        Self(rand::random())
    }
}

impl From<IdOffset> for NodeId {
    fn from(value: IdOffset) -> Self {
        NodeId(value.0)
    }
}

impl From<IdOffset> for EventId {
    fn from(value: IdOffset) -> Self {
        EventId(value.0)
    }
}
