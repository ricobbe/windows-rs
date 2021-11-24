use super::*;

pub fn gen_constant(def: &Field, gen: &Gen) -> TokenStream {
    if gen.sys {
        let name = def.name();
        let name = gen_ident(name);
        let signature = def.signature(None);

        let cfg = gen.field_cfg(def);

        if let Some(constant) = def.constant() {
            if signature.kind == constant.value_type() {
                let value = gen_constant_type_value(&constant.value());
                quote! {
                    pub const #name: #value;
                }
            } else {
                let kind = gen_sig(&signature, gen);
                let value = gen_constant_value(&constant.value());

                let value = if signature.kind.underlying_type() == constant.value_type() {
                    value
                } else {
                    quote! { #value as _ }
                };

                if signature.kind == constant.value_type() || signature.kind.is_handle() || signature.kind == ElementType::HRESULT {
                    quote! {
                        #cfg
                        pub const #name: #kind = #value;
                    }
                } else {
                    quote! {
                        #cfg
                        pub const #name: #kind = #kind(#value);
                    }
                }
            }
        } else if let Some(guid) = GUID::from_attributes(def.attributes()) {
            let guid = gen_sys_guid(&guid);
            quote! { pub const #name: ::windows_sys::core::GUID = #guid; }
        } else if let Some((guid, id)) = get_property_key(def.attributes()) {
            let kind = gen_sig(&signature, gen);
            let guid = gen_sys_guid(&guid);
            quote! {
                #cfg
                pub const #name: #kind = #kind {
                    fmtid: #guid,
                    pid: #id,
                };
            }
        } else {
            quote! {}
        }
    } else {
        quote! {}
    }
}


pub fn gen_constant_type_value(value: &ConstantValue) -> TokenStream {
    match value {
        ConstantValue::Bool(value) => quote! { bool = #value },
        ConstantValue::U8(value) => quote! { u8 = #value },
        ConstantValue::I8(value) => quote! { i8 = #value },
        ConstantValue::U16(value) => quote! { u16 = #value },
        ConstantValue::I16(value) => quote! { i16 = #value },
        ConstantValue::U32(value) => quote! { u32 = #value },
        ConstantValue::I32(value) => quote! { i32 = #value },
        ConstantValue::U64(value) => quote! { u64 = #value },
        ConstantValue::I64(value) => quote! { i64 = #value },
        ConstantValue::F32(value) => quote! { f32 = #value },
        ConstantValue::F64(value) => quote! { f64 = #value },
        ConstantValue::String(value) => quote! { &'static str = #value },
        _ => unimplemented!(),
    }
}

pub fn gen_sys_guid(guid: &GUID) -> TokenStream {
    let a = Literal::u32_unsuffixed(guid.0);
    let b = Literal::u16_unsuffixed(guid.1);
    let c = Literal::u16_unsuffixed(guid.2);
    let d = Literal::u8_unsuffixed(guid.3);
    let e = Literal::u8_unsuffixed(guid.4);
    let f = Literal::u8_unsuffixed(guid.5);
    let g = Literal::u8_unsuffixed(guid.6);
    let h = Literal::u8_unsuffixed(guid.7);
    let i = Literal::u8_unsuffixed(guid.8);
    let j = Literal::u8_unsuffixed(guid.9);
    let k = Literal::u8_unsuffixed(guid.10);

    // TODO: once code complete measure how much longer it takes if-any to use from_u128 to produce a more compact package

    quote! {
        ::windows_sys::core::GUID { data1:#a, data2:#b, data3:#c, data4:[#d, #e, #f, #g, #h, #i, #j, #k] }
    }
}


pub fn gen_constant_value(value: &ConstantValue) -> TokenStream {
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

fn get_property_key(attributes: impl Iterator<Item = Attribute>) -> Option<(GUID, u32)> {
    attributes.into_iter().find(|attribute| attribute.name() == "PropertyKeyAttribute").map(|attribute| {
        let args = attribute.args();
        (GUID::from_args(&args), args[11].1.unwrap_u32())
    })
}
