mod callback;
mod class;
mod com_interface;
mod constant;
mod delegate;
mod gen;
mod name;
mod sig;
mod r#struct;
mod winrt_interface;

use callback::*;
use class::*;
use com_interface::*;
use constant::*;
use delegate::*;
pub use gen::*;
use name::*;
use r#struct::*;
use sig::*;
use winrt_interface::*;

use quote::*;
use reader::*;

pub fn gen_types(types: &[&str], gen: &Gen) -> String {
    let reader = TypeReader::get();
    let mut tokens = String::new();

    for name in types {
        for def in reader.get_type_entry(TypeName::parse(name)).iter().flat_map(|entry| entry.def.iter()) {
            tokens.push_str(generate_type(def, gen).as_str());
        }
    }

    tokens
}

pub fn gen_namespace(_namespace: &str, _gen: &Gen) -> String {
    // TODO: code gen namespace mod assume multi-file layout
    "".to_string()
}

fn generate_type(def: &ElementType, gen: &Gen) -> TokenStream {
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
        _ => quote! {},
    }
}
