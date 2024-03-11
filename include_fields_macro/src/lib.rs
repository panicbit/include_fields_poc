use std::fs;

use anyhow::Context;
use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Fields, FieldsNamed};

#[derive(Debug, FromMeta)]
struct Args {
    path: String,
}

#[proc_macro_attribute]
pub fn include_fields(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(attr_args) => attr_args,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let fields = load_fields(&args.path).unwrap();

    let mut input = syn::parse_macro_input!(input as syn::ItemStruct);

    match &mut input.fields {
        Fields::Named(named_fields) => named_fields.named.extend(fields.named),
        Fields::Unit => input.fields = Fields::Named(fields),
        Fields::Unnamed(_) => panic!("unnamed fields are not supported"),
    }

    input.into_token_stream().into()
}

fn load_fields(path: &str) -> anyhow::Result<FieldsNamed> {
    let fields = fs::read_to_string(path).context("failed to read file")?;
    let fields = syn::parse_str::<syn::FieldsNamed>(&fields).context("failed to parse file")?;

    Ok(fields)
}
