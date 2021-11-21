use serde::Serialize;
use ts_rs::TS;

/// A single attribute belonging to a particular component.
#[derive(Serialize, TS)]
#[ts(export)]
pub struct Attribute {
    /// The attribute name, e.g. 'position' or 'mass'.
    pub name: String,
    /// The width in number of f32 values.
    pub width: usize,
}

impl Attribute {
    pub fn from_values(values: &[(&str, usize)]) -> Vec<Self> {
        values.into_iter().map(|a| Attribute::from(*a)).collect()
    }
}

impl From<(&str, usize)> for Attribute {
    fn from((name, width): (&str, usize)) -> Self {
        Self { name: name.to_string(), width }
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct Point {
    pub(self) attributes: Vec<Attribute>,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct Source {
    pub(self) attributes: Vec<Attribute>,
}

/// The descriptor of a game component's memory information.
#[derive(Serialize, TS)]
#[ts(export)]
#[serde(rename_all="lowercase")]
#[serde(tag="type")]
pub enum Component {
    Point(Point),
    Source(Source)
}

impl Component {
    pub fn point(attributes: &[(&str, usize)]) -> Self {
        Self::Point(Point {
            attributes: Attribute::from_values(attributes)
        })
    }

    pub fn source(attributes: &[(&str, usize)]) -> Self {
        Self::Source(Source {
            attributes: Attribute::from_values(attributes)
        })
    }

    fn extend_attrs(a: &mut Vec<Attribute>, b: Vec<Attribute>) {
        a.extend(b)
    }

    pub(super) fn can_merge(&self, other: &Component) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    pub(super) fn merge(&mut self, other: Component) {
        match (self, other) {
            (Component::Point(s), Component::Point(p)) => Self::extend_attrs(&mut s.attributes, p.attributes),
            (Component::Source(s), Component::Source(p)) => Self::extend_attrs(&mut s.attributes, p.attributes),
            _ => unreachable!()
        }
    }
}
