use syn::parse::{Parse, ParseStream};

pub struct KeyValue<V: Parse> {
    pub key: syn::Ident,
    pub eq_token: syn::Token![=],
    pub value: V,
}

impl<V: Parse> Parse for KeyValue<V> {
    fn parse(input: ParseStream<'_>) -> Result<Self, syn::Error> {
        Ok(KeyValue {
            key: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
