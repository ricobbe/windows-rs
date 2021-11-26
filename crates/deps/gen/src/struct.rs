use super::*;

pub fn gen_struct(def: &TypeDef, gen: &Gen) -> TokenStream {
    gen2::gen_struct(def, &gen2::Gen { namespace: gen.relative, cfg: !gen.root.is_empty(), ..Default::default() })
}

// TODO: move to test crates
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let t = TypeReader::get().expect_type_def(TypeName::new("Windows.Foundation", "Point"));
        assert_eq!(t.type_signature(), "struct(Windows.Foundation.Point;f4;f4)");
    }

    #[test]
    fn test_fields() {
        let t = TypeReader::get().expect_type_def(TypeName::new("Windows.Win32.Graphics.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA"));
        let f: Vec<Field> = t.fields().collect();
        assert_eq!(f.len(), 7);

        assert_eq!(f[0].name(), "PresentCount");
        assert_eq!(f[1].name(), "PresentRefreshCount");
        assert_eq!(f[2].name(), "SyncRefreshCount");
        assert_eq!(f[3].name(), "SyncQPCTime");
        assert_eq!(f[4].name(), "SyncGPUTime");
        assert_eq!(f[5].name(), "CompositionMode");
        assert_eq!(f[6].name(), "ApprovedPresentDuration");

        assert!(f[0].signature(None).kind == ElementType::U32);
        assert!(f[1].signature(None).kind == ElementType::U32);
        assert!(f[2].signature(None).kind == ElementType::U32);
        assert!(f[3].signature(None).kind == ElementType::I64);
        assert!(f[4].signature(None).kind == ElementType::I64);
        assert!(f[6].signature(None).kind == ElementType::U32);
    }

    #[test]
    fn test_blittable() {
        assert!(TypeReader::get().expect_type_def(TypeName::new("Windows.Foundation", "Point")).is_blittable(),);
        assert!(!TypeReader::get().expect_type_def(TypeName::new("Windows.UI.Xaml.Interop", "TypeName")).is_blittable(),);
    }
}
