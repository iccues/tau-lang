use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct MatchBody {
    match_arms: Vec<MatchArm>,
}

impl Parse for MatchBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut match_arms = vec![];
        while !content.is_empty() {
            match_arms.push(content.parse()?);
        }

        Ok(MatchBody { match_arms })
    }
}

impl MatchBody {
    pub fn expand(&self) -> TokenStream {
        let match_arms = self.match_arms.iter().map(|arm| arm.expand());
        let match_arms2 = self.match_arms.iter().map(|arm| arm.expand2());

        quote! {
            #( #match_arms )*
            #( #match_arms2 )*
        }
    }
}
