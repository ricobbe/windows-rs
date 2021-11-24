use super::*;

pub fn gen_enum(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        gen_sys_enum(def, gen)
    } else {
        gen_win_enum(def, gen)
    }
}

fn gen_sys_enum(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let underlying_type = def.underlying_type();
    let underlying_type = gen_element_name(&underlying_type, gen);

    let fields = def.fields().filter_map(|field| {
        if field.is_literal() {
            let field_name = gen_ident(field.name());
            let constant = field.constant().unwrap();
            let value = gen_constant_value(&constant.value());

            Some((field_name, value))
        } else {
            None
        }
    });

    if def.is_scoped() {
        let fields = fields.map(|(field_name, value)| {
            quote! {
                pub const #field_name: Self = Self(#value);
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
        let fields = fields.map(|(field_name, value)| {
            quote! {
                pub const #field_name: #name = #value;
            }
        });

        quote! {
            pub type #name = #underlying_type;
            #(#fields)*
        }
    }
}

pub fn gen_win_enum(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_name(def, gen);
    let underlying_type = def.underlying_type();

    // WinRT enums don't have the flags attribute but are paritioned merely based
    // on whether they are signed.
    let bitwise = matches!(underlying_type, ElementType::U32);

    // Win32 enums sadly don't use unsigned values uniformly so we need to rely
    // on the flags attribute.
    let bitwise = if bitwise || def.has_flags() {
        quote! {
            impl ::core::ops::BitOr for #name {
                type Output = Self;

                fn bitor(self, rhs: Self) -> Self {
                    Self(self.0 | rhs.0)
                }
            }
            impl ::core::ops::BitAnd for #name {
                type Output = Self;

                fn bitand(self, rhs: Self) -> Self {
                    Self(self.0 & rhs.0)
                }
            }
            impl ::core::ops::BitOrAssign for #name {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0.bitor_assign(rhs.0)
                }
            }
            impl ::core::ops::BitAndAssign for #name {
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0.bitand_assign(rhs.0)
                }
            }
            impl ::core::ops::Not for #name {
                type Output = Self;

                fn not(self) -> Self {
                    Self(self.0.not())
                }
            }
        }
    } else {
        quote! {}
    };

    let underlying_type = gen_element_name(&underlying_type, gen);
    // TODO: is this still needed or do all enums now have values?
    let mut last: Option<ConstantValue> = None;

    let fields = def.fields().filter_map(|field| {
        if field.is_literal() {
            let field_name = gen_ident(field.name());

            if let Some(constant) = field.constant() {
                let value = gen_constant_value(&constant.value());

                Some(quote! {
                    pub const #field_name: #name = #name(#value);
                })
            } else if let Some(last_value) = &last {
                let next = last_value.next();
                let value = gen_constant_value(&next);
                last = Some(next);

                Some(quote! {
                    pub const #field_name: #name = #name(#value);
                })
            } else {
                last = Some(ConstantValue::I32(0));

                Some(quote! {
                    pub const #field_name: #name = #name(0);
                })
            }
        } else {
            None
        }
    });

    let fields = if def.is_scoped() {
        quote! {
            impl #name {
                #(#fields)*
            }
        }
    } else {
        quote! {
            #(#fields)*
        }
    };

    let runtime_type = if def.is_winrt() {
        let signature = Literal::byte_string(def.type_signature().as_bytes());

        quote! {
            unsafe impl ::windows::core::RuntimeType for #name {
                const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(#signature);
            }
            impl ::windows::core::DefaultType for #name {
                type DefaultType = Self;
            }
        }
    } else {
        quote! {}
    };

    let extensions = if def.type_name() == TypeName::WIN32_ERROR {
        quote! {
            impl ::core::convert::From<WIN32_ERROR> for ::windows::core::HRESULT {
                fn from(value: WIN32_ERROR) -> Self {
                    Self(if value.0 as i32 <= 0 {
                        value.0 as _
                    } else {
                        (value.0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
                    } as _)
                }
            }
        }
    } else {
        quote!{}
    };

    quote! {
        #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default, ::core::fmt::Debug)]
        #[repr(transparent)]
        pub struct #name(pub #underlying_type);
        #fields
        impl ::core::convert::From<#underlying_type> for #name {
            fn from(value: #underlying_type) -> Self {
                Self(value)
            }
        }
        unsafe impl ::windows::core::Abi for #name {
            type Abi = Self;
        }
        #runtime_type
        #bitwise
        #extensions
    }
}
