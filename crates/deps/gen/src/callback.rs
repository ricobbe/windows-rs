use super::*;

pub fn gen_callback(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen2::gen_callback(def, &gen2::Gen { namespace: gen.relative, cfg: !gen.root.is_empty(), ..Default::default() })

}
