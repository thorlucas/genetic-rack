use wasm_bindgen::prelude::*;
use serde::Serialize;
use ts_rs::TS;

use super::{PtrBufF32, Component};

/// Represents a buffer of f32 values.
#[derive(TS, Serialize)]
#[ts(export)]
pub struct BufferF32 {
    /// The pointer to the beginning of the buffer relative to the linear memory of the wasm
    /// instance.
    ptr: PtrBufF32,

    /// The maximum number of components.
    ///
    /// Note that we do not store the stride itself as it is redundant data. 
    /// If we have an object schema for the buffer, each with its own length, then we calculate the
    /// stride and offsets that way.
    ///
    /// Instead, we will store the maximum number of *components*, which will determine the full
    /// length of the buffer.
    items: usize,

    /// Our component type.
    ///
    /// It is assumed that each component type will have a separate buffer that lives somewhere
    /// else in memory. It is possible, however, for one single component type to have multiple
    /// different buffers for each different attribute, or to have a single buffer with all
    /// attributes.
    ///
    /// Multiple buffers containing the same attribute for the same component is considered an
    /// error.
    component: Component,
}

impl BufferF32 {
    pub fn new(component: Component, items: usize, buf: &[f32]) -> Self {
        Self {
            ptr: buf.into(),
            items,
            component,
        }
    }

    pub fn merge_bufs(buffers: Vec<BufferF32>) -> Vec<BufferF32> {
        let mut out: Vec<BufferF32> = vec![];
        'outer: for i in buffers.into_iter() {
            let mut i = i;
            for b in out.iter_mut() {
                match b.try_merge(i) {
                    Err(ret) => i = ret,
                    _ => continue 'outer,
                }
            }
            out.push(i)
        }
        out
    }

    /// Tries to merge with another buffer.
    /// If it succeeds, it mutates itself and consumes the other buffer.
    /// If it cannot, it returns the other buffer back.
    fn try_merge(&mut self, other: BufferF32) -> Result<(), BufferF32> {
        if self.ptr == other.ptr && self.component.can_merge(&other.component) {
            self.items += other.items;
            self.component.merge(other.component);
            Ok(())
        } else {
            Err(other)
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
pub const INCLUDE_BINDINGS: &'static str = r#"
import type { BufferF32 } from './bindings/BufferF32';
export { BufferF32 };
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "BufferF32")]
    pub type IBufferF32;
}

impl From<BufferF32> for IBufferF32 {
    fn from(buf: BufferF32) -> Self {
        JsValue::from_serde(&buf).unwrap().into()
    }
}
