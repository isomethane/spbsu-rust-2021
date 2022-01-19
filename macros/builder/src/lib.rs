use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, GenericArgument, Ident, Lit, LitStr, Meta,
    MetaNameValue, NestedMeta, PathArguments, Type,
};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let builder_name = Ident::new("Builder", Span::call_site());

    enum FieldModifier<'a> {
        Required,
        Optional,
        Repeated {
            each_name: String,
            each_type: &'a Type,
        },
    }

    fn expand_parameterized_type(field: &Field) -> Option<(String, &Type)> {
        if let Type::Path(ref type_path) = field.ty {
            if let Some(segment) = type_path.path.segments.first() {
                if let PathArguments::AngleBracketed(ref args) = segment.arguments {
                    if let Some(&GenericArgument::Type(ref inner_ty)) = args.args.first() {
                        return Some((segment.ident.to_string(), inner_ty));
                    }
                }
            }
        }
        None
    }

    fn get_attribute(field: &Field) -> Option<(String, String)> {
        if let Some(attr) = field
            .attrs
            .iter()
            .find(|attr| attr.path.get_ident().map_or(false, |id| id == "builder"))
        {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                if let Some(NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                    path,
                    lit: Lit::Str(lit),
                    ..
                }))) = meta_list.nested.first()
                {
                    return Some((path.get_ident().unwrap().to_string(), lit.value()));
                }
            }
        }
        None
    }

    fn get_modifier(field: &Field) -> FieldModifier {
        let expanded_type = expand_parameterized_type(field);
        if let Some((ref type_name, _)) = expanded_type {
            if type_name == "Option" {
                return FieldModifier::Optional;
            }
        }
        if let Some((_, each_name)) = get_attribute(field) {
            return FieldModifier::Repeated {
                each_name,
                each_type: expanded_type.unwrap().1,
            };
        }
        FieldModifier::Required
    }

    let fields: Vec<&Field> = match input.data {
        Data::Struct(ref s) => match s.fields {
            Fields::Named(ref fields) => fields.named.iter().collect(),
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    if let Some(field) = fields
        .iter()
        .find(|f| get_attribute(f).map_or(false, |f| f.0 != "each"))
    {
        let attr = field.attrs.first().unwrap().parse_meta().unwrap();
        return syn::Error::new_spanned(attr, "expected `builder(each = \"...\")`")
            .to_compile_error()
            .into();
    }

    let field_name: Vec<&Option<Ident>> = fields.iter().map(|f| &f.ident).collect();

    let builder_field_type: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|f| {
            let ty = &f.ty;
            match get_modifier(f) {
                FieldModifier::Required => quote! { std::option::Option<#ty> },
                FieldModifier::Optional | FieldModifier::Repeated { .. } => quote! { #ty },
            }
        })
        .collect();

    let field_init: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|f| match get_modifier(f) {
            FieldModifier::Required | FieldModifier::Optional => {
                quote! { std::option::Option::None }
            }
            FieldModifier::Repeated { .. } => quote! { std::vec![] },
        })
        .collect();

    let field_build: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|f| {
            let name = &f.ident;
            match get_modifier(f) {
                FieldModifier::Required => quote! { self.#name.take().unwrap() },
                FieldModifier::Optional => quote! { self.#name.take() },
                FieldModifier::Repeated { .. } => {
                    quote! { std::mem::replace(&mut self.#name, std::vec![]) }
                }
            }
        })
        .collect();

    let required_field_check: Vec<proc_macro2::TokenStream> = fields.iter().filter_map(|f| {
        let name = &f.ident;
        let name_literal = LitStr::new(f.ident.as_ref().unwrap().to_string().as_str(), Span::call_site());
        match get_modifier(f) {
            FieldModifier::Required => Some(quote! {
                if (self.#name.is_none()) {
                    return std::result::Result::Err(String::from(format!("field {} is not set", #name_literal)).into());
                }
            }),
            _ => None,
        }
    }).collect();

    let field_setter: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .filter_map(|f| {
            let name = &f.ident;
            let modifier = get_modifier(f);
            if let FieldModifier::Repeated { ref each_name, .. } = modifier {
                if name.as_ref().unwrap() == each_name.as_str() {
                    return None;
                }
            }
            let (ty, init_value) = match modifier {
                FieldModifier::Required => (&f.ty, quote! { std::option::Option::Some(#name) }),
                FieldModifier::Optional => (
                    expand_parameterized_type(f).unwrap().1,
                    quote! { std::option::Option::Some(#name) },
                ),
                FieldModifier::Repeated { .. } => (&f.ty, quote! { #name }),
            };
            Some(quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = #init_value;
                    self
                }
            })
        })
        .collect();

    let repeated_field_setter: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .filter_map(|f| {
            let name = &f.ident;
            if let FieldModifier::Repeated {
                each_name,
                each_type,
            } = get_modifier(f)
            {
                let each_name = Ident::new(each_name.as_str(), Span::call_site());
                return Some(quote! {
                    pub fn #each_name(&mut self, #each_name: #each_type) -> &mut Self {
                        self.#name.push(#each_name);
                        self
                    }
                });
            }
            None
        })
        .collect();

    (quote! {
        impl #struct_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_name: #field_init,)*
                }
            }
        }

        pub struct #builder_name {
            #(#field_name: #builder_field_type,)*
        }

        impl #builder_name {
            #(#field_setter)*
            #(#repeated_field_setter)*

            pub fn build(&mut self) -> std::result::Result<#struct_name, std::boxed::Box<dyn std::error::Error>> {
                #(#required_field_check)*
                return std::result::Result::Ok(#struct_name {
                    #(#field_name: #field_build),*
                });
            }
        }
    }).into()
}
