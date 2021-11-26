mod bool32;
mod bstr;
mod callback;
mod class;
mod com_interface;
mod constant;
mod delegate;
mod r#enum;
mod function;
mod gen;
mod handle;
mod matrix3x2;
mod matrix4x4;
mod name;
mod ntstatus;
mod pstr;
mod pwstr;
mod sig;
mod r#struct;
mod timespan;
mod vector2;
mod vector3;
mod vector4;
mod winrt_interface;

use bool32::*;
use bstr::*;
use class::*;
use delegate::*;
pub use gen::*;
use handle::*;
use matrix3x2::*;
use matrix4x4::*;
use name::*;
use ntstatus::*;
use pstr::*;
use pwstr::*;
use sig::*;
use timespan::*;
use vector2::*;
use vector3::*;
use vector4::*;
use winrt_interface::*;

// TODO: these should not be public (just public for porting the gen crate)
pub use callback::*;
pub use com_interface::*;
pub use constant::*;
pub use function::*;
pub use r#enum::*;
pub use r#struct::*;

use quote::*;
use reader::*;

pub fn gen_type(name: &str, gen: &Gen) -> String {
    let reader = TypeReader::get();
    let mut tokens = String::new();

    for def in reader.get_type_entry(TypeName::parse(name)).iter().flat_map(|entry| entry.def.iter()) {
        tokens.push_str(gen_element_type(def, gen).as_str());
    }

    tokens
}

pub fn gen_namespace(gen: &Gen) -> String {
    let tree = TypeReader::get().get_namespace(gen.namespace).expect("Namespace not found");

    let namespaces = tree.namespaces.iter().map(move |(name, tree)| {
        if tree.namespace == "Windows.Win32.Interop" {
            return quote! {};
        }

        let name = gen_ident(name);
        let namespace = tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
        quote! {
            #[cfg(feature = #namespace)] pub mod #name;
        }
    });

    let functions = gen_functions(tree, gen);
    let types = gen_non_function_types(tree, gen);

    quote! {
        #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
        #(#namespaces)*
        #functions
        #types
    }
    .into_string()
}

fn gen_non_function_types(tree: &TypeTree, gen: &Gen) -> TokenStream {
    let mut tokens = TokenStream::new();

    for entry in tree.types.values() {
        for def in &entry.def {
            match def {
                ElementType::MethodDef(_) => {}
                ElementType::Field(_) | ElementType::TypeDef(_) => tokens.combine(&gen_element_type(def, gen)),
                _ => {}
            }
        }
    }

    tokens
}

fn gen_element_type(def: &ElementType, gen: &Gen) -> TokenStream {
    match def {
        ElementType::Field(def) => gen_constant(def, gen),
        ElementType::TypeDef(def) => match def.kind() {
            TypeKind::Class => gen_class(def, gen),
            TypeKind::Enum => gen_enum(def, gen),
            TypeKind::Struct => gen_struct(def, gen),
            TypeKind::Interface => {
                if def.is_winrt() {
                    gen_winrt_interface(def, gen)
                } else {
                    gen_com_interface(def, gen)
                }
            }
            TypeKind::Delegate => {
                if def.is_winrt() {
                    gen_delegate(def, gen)
                } else {
                    gen_callback(def, gen)
                }
            }
        },
        ElementType::MethodDef(def) => gen_function(def, gen),
        _ => quote! {},
    }
}
