use serde::Serialize;
use ts_rs::TS;

use super::AttributeDescriptor;

#[derive(Serialize, TS)]
#[ts(export)]
pub struct InterleavedAttributeDescriptor {
    // FIXME: Have to inline due to ts-rs bug
    #[ts(inline)]
    attribute: AttributeDescriptor,
    offset: usize,
    stride: usize,
}
