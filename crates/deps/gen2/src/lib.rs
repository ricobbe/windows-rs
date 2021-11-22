use reader::*;
use quote::*;

pub struct Gen<'a> {
    pub inherit: bool,
    pub sys: bool,
    pub flatten: bool,
    pub relative: &'a str,
}

pub fn generate_types(types: &[&str], gen: &Gen) -> String {
    let reader = TypeReader::get();
    let mut tokens = String::new();

    for name in types {
        for def in reader.get_type_entry(type_name(name)).iter().flat_map(|entry|entry.def.iter()) {
            tokens.push_str(generate_type(def, gen).as_str());
        }
    }

    tokens
}

pub fn generate_namespace(namespace: &str, gen: &Gen) -> String {
    // TODO: code gen namespace mod assume multi-file layout
    "".to_string()
}

fn generate_type(def: &ElementType, gen: &Gen) -> TokenStream {

    quote! {}
}

pub fn type_name(full_name: &str) -> (&str, &str) {
    let index = full_name.rfind('.').expect("Expected full name separated with `.`");
    (&full_name[0..index], &full_name[index + 1..])
}
