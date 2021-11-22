use super::*;

pub fn gen_delegate(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        let name: TokenStream = if def.generics.is_empty() {
            def.name().into()
        } else {
            let name = def.name();
            name[..name.len() - 2].into()
        };

        quote! {
            pub type #name = *mut ::core::ffi::c_void;
        }
    } else {
        quote! {}
    }
}
