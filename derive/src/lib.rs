extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::{
    parse_macro_input,
    ItemImpl,
    ImplItem,
    MethodSig,
    punctuated::{
        Punctuated,
    },
    Ident,
    Token,
    spanned::Spanned,
};

use proc_macro::{
    TokenStream
};

#[proc_macro_attribute]
pub fn gdnative_expose(_meta: TokenStream, input: TokenStream) -> TokenStream {

    // println!("{}", input.clone().to_string());

    let ast = parse_macro_input!(input as ItemImpl);

    let toks = impl_gdnative_expose(ast);

    // println!("{}", toks.clone().to_string());

    toks
}

fn impl_gdnative_expose(ast: ItemImpl) -> TokenStream {

    let mut result = ast.clone();

    result.items.clear();

    let mut methods_to_export = vec![];

    for func in ast.items {
        let item = match func {
            ImplItem::Method(mut method) => {

                let attribute_pos = method.attrs.iter()
                    .position(|attr| {
                        let correct_style = match attr.style {
                            syn::AttrStyle::Outer => true,
                            _ => false,
                        };

                        for path in attr.path.segments.iter() {
                            if path.ident.to_string() == "export" {
                                return correct_style;
                            }
                        }

                        false
                    });

                if let Some(idx) = attribute_pos {
                    let attr = method.attrs.remove(idx);

                    methods_to_export.push(method.sig.clone());
                }

                ImplItem::Method(method)
            },
            item => {
                item
            }
        };

        result.items.push(item);

    }



    for method in methods_to_export {
        let generics = method.decl.generics;

        if generics.type_params().count() > 0 {
            eprintln!("type parameters not allowed in exported functions");
            continue;
        }
        if generics.lifetimes().count() > 0 {
            eprintln!("lifetime parameters not allowed in exported functions");
            continue;
        }
        if generics.const_params().count() > 0 {
            eprintln!("const parameters not allowed in exported functions");
            continue;
        }

        let inputs = method.decl.inputs;
        let output = method.decl.output;

        print!("{}", quote!(#inputs).to_string());
        println!("{}", quote!(#output).to_string());
    }


    TokenStream::from(quote::quote!(#result))
}