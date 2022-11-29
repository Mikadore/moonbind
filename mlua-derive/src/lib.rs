use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{DeriveInput, Fields, Ident, Type};

struct Error {
    what: &'static str,
}

impl Error {
    fn err<T>(what: &'static str) -> MacroResult<T> {
        MacroResult::Err(Error { what })
    }
}

impl ToTokens for Error {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let what = self.what;
        tokens.extend(quote! {
            compile_error!(#what)
        })
    }
}

type MacroResult<T> = std::result::Result<T, Error>;

enum FieldsKind {
    Named(Vec<(Ident, Type)>),
    Unnamed(Vec<Type>),
}

struct DeriveStruct {
    ident: Ident,
    fields: FieldsKind,
}

fn derive_data(input: DeriveInput) -> MacroResult<DeriveStruct> {
    if input.generics.params.len() > 0 {
        return Error::err("This macro doesn't currently support generic structs");
    }
    let ident = input.ident;
    let data = match input.data {
        syn::Data::Struct(ds) => ds,
        _ => return Error::err("This macro currently only supports structs"),
    };

    let fields = match data.fields {
        Fields::Named(named) => {
            let named = named
                .named
                .into_iter()
                .map(|field| (field.ident.unwrap(), field.ty))
                .collect();
            FieldsKind::Named(named)
        }
        Fields::Unnamed(unnamed) => {
            let unnamed = unnamed.unnamed.into_iter().map(|field| field.ty).collect();
            FieldsKind::Unnamed(unnamed)
        }
        _ => return Error::err("This macro currently doesn't support unit structs"),
    };

    Ok(DeriveStruct { ident, fields })
}

fn gen_self_expr(fields: &FieldsKind) -> TokenStream {
    match fields {
        FieldsKind::Named(named) => {
            let bindings: TokenStream = named.iter().map(|(ident, ty)| {
                let ident_name = ident.to_string();
                quote! {
                    let #ident = <#ty as ::mlua::FromLua>::from_lua(table.get(#ident_name)?, lua)?;
                }
            }).collect();
            
            let ident_iter = named.iter().map(|tup| tup.0.clone());
            
            quote! {
                {
                    #bindings
                    Ok(Self{
                        #(#ident_iter),* 
                    })
                }
            }
        }
        FieldsKind::Unnamed(_unnamed) => {
            unimplemented!()
        }
    }
}

fn from_lua_impl(input: DeriveInput) -> MacroResult<TokenStream> {
    let data = derive_data(input)?;
    let ident = data.ident;
    let ident_name = ident.to_string();
    let self_expr = gen_self_expr(&data.fields);
    Ok(quote! {
        impl<'lua> ::mlua::FromLua<'lua> for #ident {
            fn from_lua(lua_value: ::mlua::Value<'lua>, lua: &'lua ::mlua::Lua) -> ::mlua::Result<Self> {
                match lua_value {
                    ::mlua::Value::Table(table) => {
                        #self_expr
                    }
                    _ => Err(::mlua::Error::FromLuaConversionError {
                        from: lua_value.type_name(),
                        to: #ident_name,
                        message: ::std::option::Option::Some(
                            ::std::string::String::from("Lua value must be a table!")
                        ),
                    })
                }
            }
        }
    })
}

#[proc_macro_derive(FromLua)]
pub fn derive_from_lua(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_in = syn::parse_macro_input!(item as syn::DeriveInput);

    match from_lua_impl(derive_in) {
        Ok(tokens) => tokens.into(),
        Err(e) => quote! {
            compile_error!(#e)
        }
        .into(),
    }
}


fn gen_set_fields(fields: &FieldsKind) -> TokenStream {
    match fields {
        FieldsKind::Named(named) => {
            let set_stmt: TokenStream = named.iter().map(|(ident, ty)|{
                let ident_name = ident.to_string();
                quote! { table.set(#ident_name, <#ty as ::mlua::ToLua>::to_lua(self.#ident, &lua)?)?; }
            }).collect();
            set_stmt
        }
        FieldsKind::Unnamed(_) => quote! {
            compile_error!("This macro doesn't currently support tuple structs");
        }
    }
}

fn to_lua_impl(input: DeriveInput) -> MacroResult<TokenStream> {
    let data = derive_data(input)?;
    let ident = data.ident;
    let field_setters = gen_set_fields(&data.fields);
    Ok(quote! {
        impl<'lua> ::mlua::ToLua<'lua> for #ident {
            fn to_lua(self, lua: &'lua ::mlua::Lua) -> ::mlua::Result<::mlua::Value<'lua>> {
                let table = lua.create_table()?;
                #field_setters
                Ok(::mlua::Value::Table(table))
            }
        }
    })
}

#[proc_macro_derive(ToLua)]
pub fn derive_to_lua(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_in = syn::parse_macro_input!(item as syn::DeriveInput);

    match to_lua_impl(derive_in) {
        Ok(tokens) => tokens.into(),
        Err(e) => quote! {
            compile_error!(#e)
        }
        .into(),
    }
}