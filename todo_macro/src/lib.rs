use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;
#[proc_macro_attribute]
pub fn todo_app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident.clone();

    let output = if let Data::Struct(data_struct) = &input.data {
        let mut new_fields = Vec::new();

        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                let mut new_field = field.clone();

                if let Some(ident) = &field.ident {
                    let original = ident.to_string();
                    let pascal = to_pascal_case(&original);
                    let renamed = format!("TodoApp{}", pascal);

                    new_field.attrs.push(syn::parse_quote!(
                        #[serde(rename = #renamed)]
                    ));
                }

                new_fields.push(new_field);
            }

            let vis = &input.vis;
            let generics = &input.generics;

            let output = quote! {
                #[derive(Serialize, Deserialize, Debug, Clone)]
                #vis struct #name #generics {
                    #(#new_fields),*
                }
            };

            output
        } else {
            quote!(compile_error!("todo_app only supports structs with named fields"))
        }
    } else {
        quote!(compile_error!("todo_app only supports structs"))
    };

    output.into()
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
