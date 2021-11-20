use serde::Serialize;
use ts_rs::TS;

use super::PtrBufF32;

/// Represents a buffer of f32 values.

#[derive(TS, Serialize, Copy, Clone, Debug)]
#[ts(export)]
pub struct BufferF32 {
    /// The pointer to the beginning of the buffer
    pub ptr: PtrBufF32,

    /// Note that we do not store the stride itself as it is redundant data. 
    /// If we have an object schema for the buffer, each with its own length, then we calculate the
    /// stride and offsets that way.
    ///
    /// Instead, we will store the maximum number of *components*, which will determine the full
    /// length of the buffer.
    pub items: usize,

    /// Then we have our object schema. Each object will store its length and it's attributal
    /// information. Offset is not necessary since the offset can be calculated based on the
    /// ordering of the components.
    pub components: f32,
}


