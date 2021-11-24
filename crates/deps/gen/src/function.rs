use super::*;

pub fn gen_function(def: &MethodDef, gen: &Gen) -> TokenStream {
    gen2::gen_function(def, &gen2::Gen { namespace: gen.relative, cfg: !gen.root.is_empty(), ..Default::default() })
}
