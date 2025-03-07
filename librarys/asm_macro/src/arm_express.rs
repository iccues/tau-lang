use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct ArmExpress {
    block: TokenStream,
    len: Option<LitInt>,
}

impl Parse for ArmExpress {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![#]) {
            input.parse::<Token![#]>()?;
            let len = Some(input.parse()?);

            let block;
            braced!(block in input);
            let block = block.parse()?;

            input.parse::<Token![,]>()?;

            Ok(ArmExpress { block, len })
        } else {
            let expr: Expr = input.parse()?;
            input.parse::<Token![,]>()?;
            let block = quote! {#expr};

            Ok(ArmExpress { block, len: None })
        }
    }
}

impl ArmExpress {
    pub fn expand(&self) -> TokenStream {
        match &self.len {
            Some(len) => {
                let block = &self.block;
                quote! {
                    crate::bit_vec_builder! {
                        # [#len]
                        #block
                    },
                }
            }
            None => {
                let block = &self.block;
                quote! { #block, }
            }
        }
    }
}
