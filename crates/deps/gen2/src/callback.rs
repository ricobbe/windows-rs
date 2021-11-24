use super::*;

pub fn gen_callback(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_name(def, gen);
    let method = def.invoke_method();
    let signature = method.signature(&[]);
    let return_sig = gen_return_sig(&signature, gen);
    let arch_cfg = gen.arch_cfg(def.attributes());
    let feature_cfg = gen.method_cfg(&method).0;

    let params = signature.params.iter().map(|p| {
        let name = gen_param_name(&p.param);
        let tokens = gen_abi_param_sig(p, gen);
        quote! { #name: #tokens }
    });

    let callback = quote! { 
        unsafe extern "system" fn(#(#params),*) #return_sig
    };

    // TODO: resolve difference
    let callback = if gen.sys {
        quote! { 
            ::core::option::Option<#callback>
        }
    } else {
        callback
    };

    quote! {
        #arch_cfg
        #feature_cfg
        pub type #name = #callback;
    }
}
