use proc_macro::TokenStream;
use quote::quote;
use ts_rs::TS;

use abi::{Attribute, AttributeDescriptor, InterleavedAttributeDescriptor};

macro_rules! export {
    ($($t:ty),*$(,)?) => {
        export!(@emit $(
            (
                <$t>::decl();
                "export {}\n"
            )
        ),*)
    };
    (@emit $(($e:expr ; $fmt:literal)),*) => {
        format!(concat!($($fmt),*), $($e),*)
    };
}


#[proc_macro]
pub fn generate_typescript(_: TokenStream) -> TokenStream {
    let exports = export! {
        Attribute,
        AttributeDescriptor,
        InterleavedAttributeDescriptor
    };

    TokenStream::from(quote! {
        #[wasm_bindgen(typescript_custom_section)]
        const DECLARATIONS: &'static str = #exports;
    })
}

#[cfg(test)]
mod test {
    use ts_rs::TS;
    use abi::{Attribute, AttributeDescriptor, InterleavedAttributeDescriptor};

    #[test]
    pub fn test_export() {
        let exports = export! {
            Attribute,
            AttributeDescriptor,
            InterleavedAttributeDescriptor,
        };

        println!("{}", exports);
    }
}
