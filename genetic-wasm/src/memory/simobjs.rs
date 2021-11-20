use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(rename_all="lowercase")]
pub enum SimObj {
    Point(f32),
    Source(f32),
}

#[derive(Serialize, TS)]
#[ts(export)]
#[ts(rename_all="lowercase")]
#[serde(tag="type")]
pub enum ObjAttr {
    Position {
        #[ts(type="3")]
        width: usize
    },
    Mass {
        #[ts(type="1")]
        width: usize
    },
}
