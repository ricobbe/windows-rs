use super::*;

pub fn gen_enum(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        quote! {}
    } else {
        quote! {}
    }
}
