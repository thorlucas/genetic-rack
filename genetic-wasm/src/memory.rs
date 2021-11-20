use wasm_bindgen::prelude::*;
use serde::Serialize;
use typescript_definitions_ufo_patch::TypescriptDefinition;

/// Wrapper around the pointer that gets serialized to an offset.
/// TODO: Double check that this is the same offset wasm_bindgen produces.
#[derive(Copy, Clone, Debug)]
#[wasm_bindgen]
pub struct PtrBufferF32(*const f32);

impl<T> From<T> for PtrBufferF32 where T: AsRef<[f32]> {
    fn from(v: T) -> Self {
        let slice = v.as_ref();
        Self(slice.as_ptr() as *const f32)
    }
}

impl Serialize for PtrBufferF32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_u32(self.0 as u32)
    }
}

#[derive(Serialize, TypescriptDefinition)]
#[serde(tag = "type")]
/// Represents a buffer that needs to be created.
pub enum BufferF32 {
    #[serde(rename = "single")]
    Single {
        /// Pointer into the linear memory of the buffer.
        ptr: PtrBufferF32,

        /// The number of total items.
        items: usize,

        /// The object which appears in the buffer.
        object: Object,
    },
    #[serde(rename = "inter")]
    Interleaved {
        /// Pointer into the linear memory of the buffer.
        ptr: PtrBufferF32,
    }
}

#[derive(Serialize, TypescriptDefinition)]
#[serde(tag = "type")]
/// The object types we expose
pub enum Object {
    #[serde(rename = "point")]
    Point { foo: f32 },
    #[serde(rename = "source")]
    Source { bar: String },
}
