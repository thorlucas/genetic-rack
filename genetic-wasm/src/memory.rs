use std::collections::HashMap;

use serde::Serialize;
use typescript_definitions_ufo_patch::TypescriptDefinition;
use wasm_bindgen::prelude::*;

// POINTER

/// Wrapper around the pointer that gets serialized to an offset.
#[derive(Copy, Clone, Debug)]
#[wasm_bindgen]
pub struct BufferF32Ptr(*const f32);

impl<T> From<T> for BufferF32Ptr
where
    T: AsRef<[f32]>,
{
    fn from(v: T) -> Self {
        let slice = v.as_ref();
        Self(slice.as_ptr() as *const f32)
    }
}

impl Serialize for BufferF32Ptr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0 as u32)
    }
}

// BUFFER

#[derive(Serialize, TypescriptDefinition)]
/// Represents a buffer that needs to be created.
pub struct BufferF32 {
    /// Pointer into the linear memory of the buffer.
    ptr: BufferF32Ptr,

    /// The number of total items.
    items: usize,

    /// This will be an ordered list of the objects which appear in the buffer.
    /// The stride and offsets can be calculated based on this list, so we don't include it in
    /// order to avoid introducing any bugs.
    object: BufType,
}

#[derive(Serialize)]
#[serde(untagged)]
enum BufType {
    Single(f32),
    Inter(Vec<f32>),
}

#[derive(Serialize, TypescriptDefinition)]
/// The object types we expose. These type names are also used to look up what kind of vertices
/// they're allowed to have.
pub enum SimObject {
    Point,
    Source,
}

#[derive(Serialize, TypescriptDefinition)]
// We want to tag externally so that we get a type map.
/// The attributes allowed for a particular type.
pub enum SimObjectAttributes {
    #[serde(rename = "point")]
    Point(Position),
    #[serde(rename = "source")]
    Source(Position, Mass)
}

// Marker types for attributes
#[derive(Serialize, TypescriptDefinition)]
#[serde(rename = "position")]
pub struct Position(SimObject);

#[derive(Serialize, TypescriptDefinition)]
#[serde(rename = "mass")]
pub struct Mass;

