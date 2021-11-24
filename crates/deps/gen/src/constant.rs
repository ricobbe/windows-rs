use super::*;

pub fn gen_constant(def: &Field, gen: &Gen) -> TokenStream {
    gen2::gen_constant(def, &gen2::Gen {
        namespace: gen.relative,
        cfg: !gen.root.is_empty(),
        ..Default::default()
    })
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
