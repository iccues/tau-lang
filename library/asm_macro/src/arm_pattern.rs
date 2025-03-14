use syn::Path;

use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct ArmPattern {
    code: Path,
    args: Punctuated<Ident, Token![,]>,
}

impl Parse for ArmPattern {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);

        let code = content.parse()?;
        content.parse::<Token![,]>()?;

        let inner_content;
        bracketed!(inner_content in content);
        let args = inner_content.parse_terminated(Ident::parse)?;

        Ok(ArmPattern { code, args })
    }
}

impl ArmPattern {
    pub fn expand(&self) -> TokenStream {
        let code = &self.code;
        let args = self.args.iter();

        quote! {
            (
                code @ #code,
                args @ [
                    #( Arg::#args(_), )*
                ],
            )
        }
    }

    pub fn expand2(&self) -> TokenStream {
        let code = &self.code;

        quote! {
            (#code, _)
        }
    }
}
