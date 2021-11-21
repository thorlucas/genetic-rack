use ts_rs::TS;
use serde::Serialize;

/// A pointer to a f32 buffer somewhere in linear memory.

#[derive(TS, Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[ts(export)]
pub struct PtrBufF32(usize);

impl<T> From<T> for PtrBufF32
    where T: AsRef<[f32]>
{
    fn from(f32arr: T) -> Self {
        let slc: &[f32] = f32arr.as_ref();
        let ptr: *const f32 = slc.as_ptr();
        Self(ptr as usize)
    }
}

//impl<T> From<T> for PtrBufF32 where T: AsRef<[[f32; 3]]> {
    //fn from(f32arr: T) -> Self {
        //let slc: &[[f32; 3]] = f32arr.as_ref();
        //let ptr = slc.as_ptr();
        //Self(ptr as usize)
    //}
//}



#[cfg(test)]
mod tests {
    use super::PtrBufF32; 
    // TODO: Test using allocators

    #[test]
    pub fn f32_array_to_buffer() {
        let arr: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
        let ptr = PtrBufF32::from(arr);
        assert_ne!(0x0, ptr.0);
    }

    //#[test]
    //pub fn f32_vec_to_buffer() {
        //let vec: Vec<f32> = (1u8..100).map(f32::from).collect();
        //let ptr = PtrBufF32::from(vec.as_ref());
        //assert_ne!(0x0, ptr.0);
    //}

    //#[test]
    //pub fn f32_vec3_to_buffer() {
        //let vec = Vec3::new(10.0, -3.0, 5.0);
        //let ptr = PtrBufF32::from(vec);
        //assert_ne!(0x0, ptr.0);
    //}


    //#[test]
    //pub fn f32_arr3_to_buffer() {
        //let vec: Vec<[f32; 3]> = (1u8..20).map(f32::from).map(|n| [1.0 * n, -1.0 * n, 5.0 * n]).collect();
        //let ptr = PtrBufF32::from(vec);
        //assert_ne!(0x0, ptr.0);
    //}

    //#[test]
    //pub fn f32_vec3_to_buffer() {
        //let vec: Vec<Vec3> = (1u8..100).map(f32::from).map(|n| Vec3::new(n * 0.2, n, n + 0.5)).collect();
        //let ptr = PtrBufF32::from(vec);
        //assert_ne!(0x0, ptr.0);
    //}
}
