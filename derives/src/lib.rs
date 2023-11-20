// extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CanvasGroupDropMacro)]
pub fn canvas_group_drop_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let expanded = quote! {
        impl Drop for #struct_name {
            fn drop(&mut self) {
                self.children.iter_mut().for_each(|child| {
                    child.set_parent(None);
                });
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(CanvasGroupMacro)]
pub fn canvas_group_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let expanded = quote! {
        impl CanvasComponentGroup for #struct_name {
            fn add_child(&mut self, child: Box<dyn CanvasComponent>) {
                self.children.push(child);
            }
            fn set_translate(&mut self, x: f64, y: f64) {
                self.translate = (x, y);
            }
            fn get_translate(&self) -> (f64, f64) {
                return self.translate;
            }
        }
    };

    TokenStream::from(expanded)
}


