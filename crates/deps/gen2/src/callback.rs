use super::*;

pub fn gen_callback(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_name(def, gen);
    let signature = def.invoke_method().signature(&[]);
    let return_sig = gen_return_sig(&signature, gen);
    let cfg = quote! {};// gen.gen_function_cfg(def.attributes(), &signature);

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_param_sig(p, gen);
        quote! { #name: #tokens }
    });

    quote! {
        #cfg
        pub type #name = ::core::option::Option<unsafe extern "system" fn(#(#params),*) #return_sig>;
    }
}
