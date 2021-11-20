use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use serde::Serialize;

/// Responsible for passing to JS the memory layout in a way that it can easily interpret it.

/// Represents a buffer that needs to be created.
#[wasm_bindgen]
pub struct BufferF32 {
    /// The pointer from memory to the buffer
    pub ptr: *const f32,

    /// The size of each individual item.
    /// If this is an interleaved buffer, it's going to be the size of the entire struct.
    pub item_size: usize,

    /// This is the number of items.
    /// This combined with the `item_size` gives us the total buffer length.
    pub items: usize,

    /// Whether or not this is interleaved.
    /// If it is, then the stride is given by the `item_size`.
    pub iterleaved: bool,
}

/// The total scheme of all the data in the app.
#[wasm_bindgen]
pub struct AppDataScheme {
    /// This is the list of buffers that must be created. You will need to index into
    /// this list to retrieve the appropriate buffers for each item.
    pub buffers: Vec<BufferF32>,

    /// The individual objects. Usually this is going to mean a separate mesh or
    /// instanced mesh.
    pub objects: HashMap<String, String>,
}

/// The object types we expose
#[wasm_bindgen]
pub enum AppObject {
    Point = "Point",
    Source = "Source",
}


