mod arm_express;
mod arm_pattern;
mod code_match;
mod match_arm;
mod match_body;
mod prelude;

use prelude::*;

#[proc_macro]
pub fn code_rules(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_macro_input!(input as CodeMatch).expand().into()
}
