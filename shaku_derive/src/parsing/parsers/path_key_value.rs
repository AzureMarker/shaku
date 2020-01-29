use syn::parse::{Parse, ParseStream};

pub struct PathKeyValue {
    pub key: syn::Path,
    pub eq_token: syn::Token![=],
    pub value: syn::Path,
}

impl Parse for PathKeyValue {
    fn parse(input: ParseStream<'_>) -> Result<Self, syn::Error> {
        Ok(PathKeyValue {
            key: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
