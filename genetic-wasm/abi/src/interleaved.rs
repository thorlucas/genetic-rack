use serde::Serialize;
use ts_rs::TS;

use super::AttributeDescriptor;

#[derive(Serialize, TS)]
#[ts(export)]
pub struct InterleavedAttributeDescriptor {
    attribute: AttributeDescriptor,
    offset: usize,
    stride: usize,
}
