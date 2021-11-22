use super::*;

pub fn gen_com_interface(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        let name: TokenStream = def.name().into();

        quote! {
            pub type #name = *mut ::core::ffi::c_void;
        }
    } else {
        quote! {}
    }
}
