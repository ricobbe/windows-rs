use super::*;

pub fn gen_enum(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    gen2::gen_enum(def, &gen2::Gen {
        namespace: gen.relative,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let t = TypeReader::get().expect_type_def(TypeName::new("Windows.Foundation", "AsyncStatus"));
        assert_eq!(t.type_signature(), "enum(Windows.Foundation.AsyncStatus;i4)");
    }
}
