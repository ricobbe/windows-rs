use super::*;

pub fn gen_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    gen_sig_with_const(sig, gen, sig.is_const)
}

pub fn gen_param_sig(param: &MethodParam, gen: &Gen) -> TokenStream {
    gen_sig_with_const(&param.signature, gen, !param.param.flags().output())
}

pub fn gen_abi_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    gen_abi_sig_with_const(sig, gen, sig.is_const)
}

pub fn gen_abi_param_sig(param: &MethodParam, gen: &Gen) -> TokenStream {
    gen_abi_sig_with_const(&param.signature, gen, !param.param.flags().output())
}

pub fn gen_return_sig(signature: &MethodSignature, gen: &Gen) -> TokenStream {
    if let Some(return_sig) = &signature.return_sig {
        let tokens = gen_sig(return_sig, gen);
        quote! { -> #tokens }
    } else {
        quote! {}
    }
}

pub fn gen_method_constraints(params: &[MethodParam], gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::with_capacity();

    for (position, param) in params.iter().enumerate() {
        if param.is_convertible() {
            let name = format_token!("Param{}", position);
            let into = gen_element_name(&param.signature.kind, gen);
            tokens.combine(&quote! { #name: ::windows::core::IntoParam<'a, #into>, });
        }
    }

    if !tokens.is_empty() {
        quote! { 'a, #tokens }
    } else {
        TokenStream::new()
    }
}

pub fn gen_win32_abi_arg(param: &MethodParam) -> TokenStream {
    let name = gen_param_name(&param.param);

    if param.is_convertible() {
        quote! { #name.into_param().abi() }
    } else {
        quote! { ::core::mem::transmute(#name) }
    }
}

pub fn gen_win32_params(params: &[MethodParam], gen: &Gen) -> TokenStream {
    params
        .iter()
        .enumerate()
        .map(|(position, param)| {
            let name = gen_param_name(&param.param);

            if param.is_convertible() {
                let into = format_token!("Param{}", position);
                quote! { #name: #into, }
            } else {
                let tokens = gen_param_sig(param, gen);
                quote! { #name: #tokens, }
            }
        })
        .collect()
}

pub fn gen_win32_result_type(signature: &MethodSignature, gen: &Gen) -> TokenStream {
    let mut return_param = signature.params[signature.params.len() - 1].clone();

    if return_param.signature.pointers > 1 {
        return_param.signature.pointers -= 1;
        gen_param_sig(&return_param, gen)
    } else {
        gen_element_name(&return_param.signature.kind, gen)
    }
}

fn gen_abi_sig_with_const(sig: &Signature, gen: &Gen, is_const: bool) -> TokenStream {
    let mut tokens = TokenStream::with_capacity();

    for _ in 0..sig.pointers {
        if is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    tokens.combine(&gen_abi_element_name(&sig.kind, gen));
    tokens
}

fn gen_sig_with_const(sig: &Signature, gen: &Gen, is_const: bool) -> TokenStream {
    let mut tokens = TokenStream::with_capacity();

    for _ in 0..sig.pointers {
        if is_const {
            tokens.combine(&quote! { *const });
        } else {
            tokens.combine(&quote! { *mut });
        }
    }

    let kind = gen_element_name(&sig.kind, gen);

    // TODO: harmonize these across sys/win
    if sig.kind.is_nullable() && !gen.sys {
        tokens.combine(&quote! {
            ::core::option::Option<#kind>
        });
    } else {
        tokens.combine(&kind)
    }

    tokens
}
