use serde::Serializer;

pub fn serialize<T, S>(_: &T, _: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    panic!("Serializing never type!")
}
