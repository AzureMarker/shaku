use syn::parse::{Parse, ParseStream};

pub struct KeyValue<V: Parse> {
    pub key: syn::Ident,
    pub value: V,
}

impl<V: Parse> Parse for KeyValue<V> {
    fn parse(input: ParseStream<'_>) -> Result<Self, syn::Error> {
        let key = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let value = input.parse()?;

        Ok(KeyValue { key, value })
    }
}
