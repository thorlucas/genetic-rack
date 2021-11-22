use serde::Serialize;
use ts_rs::TS;
use super::{AttributeDescriptor, InterleavedAttributeDescriptor, never};

#[derive(Serialize, TS)]
pub struct Float32Array {
    offset: usize
}

#[derive(Serialize, TS)]
#[ts(export)]
#[serde(tag="type")]
#[serde(rename_all="lowercase")]
pub enum AttributeBuffer {
    Single { 
        buffer: Float32Array,
        attribute: AttributeDescriptor,
    },
    Interleaved {
        buffer: Float32Array,
        attributes: Vec<InterleavedAttributeDescriptor>,
    },
}
