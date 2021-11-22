use js_sys::Float32Array;
use serde::Serialize;
use ts_rs::TS;
use super::{AttributeDescriptor, InterleavedAttributeDescriptor, never};

#[derive(Serialize, TS)]
#[serde(untagged)]
pub enum BufferType {
    Offset {
        offset: usize,
    },
    Array {
        #[ts(type="Float32Array")]
        #[serde(with="never")]
        array: Float32Array,
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
#[serde(tag="type")]
#[serde(rename_all="lowercase")]
pub enum AttributeBuffer {
    Single { 
        #[ts(inline)]
        buffer: BufferType,
        // FIXME: Have to inline as a workaround for ts-rs bug that does not import
        #[ts(inline)]
        attribute: AttributeDescriptor,
    },
    Interleaved {
        #[ts(inline)]
        buffer: BufferType,
        #[ts(inline)]
        attributes: Vec<InterleavedAttributeDescriptor>,
    },
}
