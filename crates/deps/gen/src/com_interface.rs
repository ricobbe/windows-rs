use super::*;

pub fn gen_com_interface(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    if include == TypeInclude::Full {
        gen2::gen_com_interface(def, &gen2::Gen { namespace: gen.relative, cfg: !gen.root.is_empty(), ..Default::default() })
    } else {
        let name = gen_type_name(def, gen);
        let guid = gen_type_guid(def, gen);

        quote! {
            #[repr(transparent)]
            #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone, ::core::fmt::Debug)]
            #[doc(hidden)]
            pub struct #name(pub ::windows::core::IUnknown);
            unsafe impl ::windows::core::Interface for #name {
                type Vtable = <::windows::core::IUnknown as ::windows::core::Interface>::Vtable;
                const IID: ::windows::core::GUID = #guid;
            }
        }
    }
}

fn gen_method(vtable_offset: usize, method: &MethodDef, method_names: &mut BTreeMap<String, u32>, gen: &Gen) -> TokenStream {
    let signature = method.signature(&[]);
    let constraints = gen_method_constraints(&signature.params, gen);
    let vtable_offset = Literal::usize_unsuffixed(vtable_offset + 3);

    let name = method.rust_name();
    let overload = method_names.entry(name.to_string()).or_insert(0);
    *overload += 1;

    let name: TokenStream = if *overload > 1 { format_token!("{}{}", name, overload) } else { to_ident(&name) };

    let features = signature.method_features();
    let cfg = gen.gen_cfg(&features);
    let doc = gen.gen_cfg_doc(&features);

    match signature.kind() {
        SignatureKind::Query => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params) -> ::windows::core::Result<T> {
                    let mut result__ = ::core::option::Option::None;
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
                }
            }
        }
        SignatureKind::QueryOptional => {
            let leading_params = &signature.params[..signature.params.len() - 2];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints T: ::windows::core::Interface>(&self, #params result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
                }
            }
        }
        SignatureKind::ResultValue => {
            let leading_params = &signature.params[..signature.params.len() - 1];
            let args = leading_params.iter().map(gen_win32_abi_arg);
            let params = gen_win32_params(leading_params, gen);
            let return_type_tokens = gen_win32_result_type(&signature, gen);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<#return_type_tokens> {
                    let mut result__: <#return_type_tokens as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)* &mut result__)
                    .from_abi::<#return_type_tokens>(result__ )
                }
            }
        }
        SignatureKind::ResultVoid => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints>(&self, #params) -> ::windows::core::Result<()> {
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)*).ok()
                }
            }
        }
        SignatureKind::ReturnStruct => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let return_sig = gen_abi_type_name(&signature.return_sig.unwrap().kind, gen);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints>(&self, #params) -> #return_sig {
                    let mut result__: #return_sig = ::core::default::Default::default();
                    (::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), &mut result__ #(,#args)*);
                    result__
                }
            }
        }
        SignatureKind::PreserveSig => {
            let params = gen_win32_params(&signature.params, gen);
            let args = signature.params.iter().map(gen_win32_abi_arg);
            let return_sig = gen_win32_return_sig(&signature, gen);

            quote! {
                #cfg
                #doc
                pub unsafe fn #name<#constraints>(&self, #params) #return_sig {
                    ::core::mem::transmute((::windows::core::Interface::vtable(self).#vtable_offset)(::core::mem::transmute_copy(self), #(#args,)*))
                }
            }
        }
    }
}
