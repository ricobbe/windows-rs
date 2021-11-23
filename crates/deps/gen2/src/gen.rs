use super::*;

pub struct Gen<'a> {
    pub relative: &'a str,
    pub inherit: bool,
    pub sys: bool,
    pub flatten: bool,
    pub cfg: bool,
}

impl Gen<'_> {
    pub(crate) fn namespace(&self, namespace: &str) -> TokenStream {
        if self.flatten || namespace == self.relative {
            quote! {}
        } else {
            let mut relative = self.relative.split('.').peekable();
            let mut namespace = namespace.split('.').peekable();

            while relative.peek() == namespace.peek() {
                if relative.next().is_none() {
                    break;
                }

                namespace.next();
            }

            let mut tokens = TokenStream::with_capacity();

            for _ in 0..relative.count() {
                tokens.push_str("super::");
            }

            for namespace in namespace {
                tokens.push_str(namespace);
                tokens.push_str("::");
            }

            tokens
        }
    }

    pub(crate) fn type_cfg(&self, def: &TypeDef) -> TokenStream {
        if !self.cfg {
            quote! {}
        } else {
            quote! {}
        }
    }

    pub(crate) fn field_cfg(&self, def: &Field) -> TokenStream {
        if !self.cfg {
            quote! {}
        } else {
            quote! {}
        }
    }

    pub(crate) fn method_cfg(&self, def: &MethodDef) -> TokenStream {
        if !self.cfg {
            quote! {}
        } else {
            quote! {}
        }
    }
}

fn gen_arch_cfg(attributes: impl Iterator<Item = Attribute>) -> TokenStream {
    for attribute in attributes {
        if attribute.name() == "SupportedArchitectureAttribute" {
            if let Some((_, ConstantValue::I32(value))) = attribute.args().get(0) {
                let mut cfg = "#[cfg(any(".to_string();
                if value & 1 == 1 {
                    cfg.push_str(r#"target_arch = "x86", "#);
                }
                if value & 2 == 2 {
                    cfg.push_str(r#"target_arch = "x86_64", "#);
                }
                if value & 4 == 4 {
                    cfg.push_str(r#"target_arch = "aarch64", "#);
                }
                cfg.push_str("))]");
                return cfg.into();
            }
        }
    }

    quote! {}
}

fn element_requirements(def: &ElementType, namespaces: &mut std::collections::BTreeSet<&'static str>, keys: &mut std::collections::HashSet<Row>) {
    match def {
        ElementType::TypeDef(def) => type_requirements(def, namespaces, keys),
        ElementType::Array((signature, _)) => element_requirements(&signature.kind, namespaces, keys),
        _ => {}
    }
}

fn type_requirements(def: &TypeDef, namespaces: &mut std::collections::BTreeSet<&'static str>, keys: &mut std::collections::HashSet<Row>) {
    if !keys.insert(def.row.clone()) {
        return;
    }

    let namespace = def.namespace();

    if !namespace.is_empty() {
        namespaces.insert(def.namespace());
    }

    for generic in &def.generics {
        element_requirements(generic, namespaces, keys);
    }

    match def.kind() {
        TypeKind::Class => {
            if let Some(def) = def.default_interface() {
                namespaces.insert(def.namespace());
            }
        }
        TypeKind::Struct => {
            def.fields().for_each(|field| field_requirements(&field, Some(def), namespaces, keys));

            // TODO: needed?
            if let Some(def) = def.is_convertible_to() {
                namespaces.insert(def.type_name().namespace);
            }
        }
        TypeKind::Delegate => method_requirements(&def.invoke_method().signature(&[]), namespaces, keys),
        _ => {}
    }

    if let Some(entry) = TypeReader::get().get_type_entry(def.type_name()) {
        for def in &entry.def {
            if let ElementType::TypeDef(def) = def {
                type_requirements(def, namespaces, keys);
            }
        }
    }
}

fn method_requirements(def: &MethodSignature, namespaces: &mut BTreeSet<&'static str>, keys: &mut std::collections::HashSet<Row>) {
    def.return_sig.iter().for_each(|def| element_requirements(&def.kind, namespaces, keys));
    def.params.iter().for_each(|def| element_requirements(&def.signature.kind, namespaces, keys));
}

fn field_requirements(def: &Field, enclosing: Option<&TypeDef>, namespaces: &mut BTreeSet<&'static str>, keys: &mut std::collections::HashSet<Row>) {
    element_requirements(&def.signature(enclosing).kind, namespaces, keys);
}
