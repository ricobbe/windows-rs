use super::*;

pub fn gen_constant(def: &Field, gen: &Gen) -> TokenStream {
    quote! {}
}

pub fn gen_enum(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        gen_sys_enum(def, gen)
    } else {
        quote! {}
    }
}

fn gen_sys_enum(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let underlying_type = def.underlying_type();
    let underlying_type = gen_element_name(&underlying_type, gen);

    if def.is_scoped() {
        let fields = def.fields().filter_map(|field| {
            if field.is_literal() {
                let field_name = gen_ident(field.name());
                let constant = field.constant().unwrap();
                let value = gen_constant_value(&constant.value());

                Some(quote! {
                    pub const #field_name: Self = Self(#value);
                })
            } else {
                None
            }
        });

        quote! {
            #[repr(transparent)]
            pub struct #name(pub #underlying_type);
            impl #name {
                #(#fields)*
            }
            impl ::core::marker::Copy for #name {}
            impl ::core::clone::Clone for #name {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    } else {
        let fields = def.fields().filter_map(|field| {
            if field.is_literal() {
                let field_name = gen_ident(field.name());
                let constant = field.constant().unwrap();
                let value = gen_constant_value(&constant.value());

                Some(quote! {
                    pub const #field_name: #name = #value;
                })
            } else {
                None
            }
        });

        quote! {
            pub type #name = #underlying_type;
            #(#fields)*
        }
    }
}

fn gen_constant_value(value: &ConstantValue) -> TokenStream {
    match value {
        ConstantValue::Bool(value) => quote! { #value },
        ConstantValue::U8(value) => quote! { #value },
        ConstantValue::I8(value) => quote! { #value },
        ConstantValue::U16(value) => quote! { #value },
        ConstantValue::I16(value) => quote! { #value },
        ConstantValue::U32(value) => quote! { #value },
        ConstantValue::I32(value) => quote! { #value },
        ConstantValue::U64(value) => quote! { #value },
        ConstantValue::I64(value) => quote! { #value },
        ConstantValue::F32(value) => quote! { #value },
        ConstantValue::F64(value) => quote! { #value },
        ConstantValue::String(value) => quote! { #value },
        _ => unimplemented!(),
    }
}
