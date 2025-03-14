use crate::prelude::*;

#[derive(Debug)]
pub(crate) struct MatchArm {
    arm_pattern: ArmPattern,
    arm_express: ArmExpress,
}

impl Parse for MatchArm {
    fn parse(input: ParseStream) -> Result<Self> {
        let arm_pattern = input.parse()?;
        input.parse::<Token![=>]>()?;
        let arm_express = input.parse()?;

        Ok(MatchArm {
            arm_pattern,
            arm_express,
        })
    }
}

impl MatchArm {
    pub fn expand(&self) -> TokenStream {
        let arm_pattern = self.arm_pattern.expand();
        let arm_express = self.arm_express.expand();
        quote! {
            #arm_pattern => #arm_express
        }
    }

    pub fn expand2(&self) -> TokenStream {
        let arm_pattern = self.arm_pattern.expand2();
        quote! {
            #[allow(unreachable_patterns)]
            #arm_pattern => panic!(),
        }
    }
}
