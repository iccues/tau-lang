use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct CodeMatch {
    scrutinee: TokenTree,
    match_body: MatchBody,
}

impl Parse for CodeMatch {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![match]>()?;
        let scrutinee = input.parse::<TokenTree>()?;
        let match_body = input.parse()?;

        Ok(CodeMatch {
            scrutinee,
            match_body,
        })
    }
}

impl CodeMatch {
    pub fn expand(&self) -> TokenStream {
        let scrutinee = &self.scrutinee;
        let match_body = self.match_body.expand();
        quote! {
            match #scrutinee {
                #match_body
            }
        }
    }
}
