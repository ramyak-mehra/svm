#![feature(log_syntax)]

use proc_macro::{self, TokenStream};
use syn;

#[proc_macro_derive(Instruction)]
pub fn instruction_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_instruction_macro(&ast)
}

fn impl_instruction_macro(ast: &syn::DeriveInput) -> TokenStream {
    let mut content = String::new();
    if let syn::Data::Enum(e) = &ast.data {
        for v in &e.variants {
            let v = format!(
                "pub const {}: Value = Value::{}({}::{});",
                v.ident.to_string().to_uppercase(),
                ast.ident,
                ast.ident,
                v.ident
            );
            content.push_str(&v);
            content.push_str("\n");
        }
    };

    content.parse().unwrap()
}
