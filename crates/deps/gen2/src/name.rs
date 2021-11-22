
use super::*;

pub fn gen_ident(name: &str) -> TokenStream {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    match name {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield" | "try" | "async" | "await" | "dyn" => {
            format!("r#{}", name).into()
        }
        "Self" | "self" => format!("{}_", name).into(),
        "_" => "unused".into(),
        _ => name.into(),
    }
}

pub fn gen_generic_ident(name: &str) -> TokenStream {
    let len = name.len();
    let len = name.get(len - 2).map_or_else(||len, |c| if c == '`' { len - 2} else { len });
    gen_ident(&name[..len])
}

pub fn gen_element_name(def: &ElementType, gen: &Gen) -> TokenStream {
    match def {
        ElementType::Void => quote! { ::core::ffi::c_void },
        ElementType::Bool => quote! { bool },
        ElementType::Char => quote! { u16 },
        ElementType::I8 => quote! { i8 },
        ElementType::U8 => quote! { u8 },
        ElementType::I16 => quote! { i16 },
        ElementType::U16 => quote! { u16 },
        ElementType::I32 => quote! { i32 },
        ElementType::U32 => quote! { u32 },
        ElementType::I64 => quote! { i64 },
        ElementType::U64 => quote! { u64 },
        ElementType::F32 => quote! { f32 },
        ElementType::F64 => quote! { f64 },
        ElementType::ISize => quote! { isize },
        ElementType::USize => quote! { usize },
        ElementType::String => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::HSTRING }
        }
        ElementType::IInspectable => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::IInspectable }
        }
        ElementType::GUID => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::GUID }
        }
        ElementType::IUnknown => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::IUnknown }
        }
        ElementType::HRESULT => {
            let crate_name = gen_crate_name(gen);
            quote! { ::#crate_name::core::HRESULT }
        }
        ElementType::Array((kind, len)) => {
            let name = gen_sig(kind, gen);
            let len = Literal::u32_unsuffixed(*len);
            quote! { [#name; #len] }
        }
        ElementType::GenericParam(generic) => generic.into(),
        ElementType::MethodDef(def) => def.name().into(),
        ElementType::Field(field) => field.name().into(),
        ElementType::TypeDef(t) => gen_type_name(t, gen),
        _ => unimplemented!(),
    }
}

fn gen_type_name(def: &TypeDef, gen: &Gen) -> TokenStream {
    
}

pub fn gen_sig(sig: &Signature, gen: &Gen) -> TokenStream {
    gen_sig_with_const(sig, gen, sig.is_const)
}

pub fn gen_param(param: &MethodParam, gen: &Gen) -> TokenStream {
    gen_sig_with_const(&param.signature, gen, !param.param.flags().output())
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

    tokens.combine(&gen_name(&sig.kind, gen));
    tokens
}

fn gen_crate_name(gen: &Gen) -> TokenStream {
    if gen.sys {
        "windows_sys".into()
    } else {
        "windows".into()
    }
}
