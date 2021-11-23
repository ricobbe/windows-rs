use super::*;

pub struct Gen<'a> {
    pub inherit: bool,
    pub sys: bool,
    pub flatten: bool,
    pub relative: &'a str,
}

impl Gen<'_> {
    pub fn namespace(&self, namespace: &str) -> TokenStream {
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
}