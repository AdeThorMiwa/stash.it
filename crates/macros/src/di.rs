use darling::{Error, FromMeta, ast::NestedMeta};
use proc_macro::{ TokenStream};
use quote::quote;
use syn::{parse_macro_input, parse_quote, Expr, Ident, ItemStruct};

pub fn initialize_container_impl(input: TokenStream) -> TokenStream {
    let func_name = parse_macro_input!(input as Ident);

    quote! {
        use di::{ServiceCollection, ServiceDescriptor, ServiceProvider};
        use once_cell::sync::Lazy;
        use send_wrapper::SendWrapper;
        use std::sync::{Arc, Mutex};

        pub struct DIStore(Mutex<SendWrapper<ServiceCollection>>);
        pub struct DIProvider(pub Arc<ServiceProvider>);

        impl DIStore {
            pub fn new() -> Self {
                Self(Mutex::new(SendWrapper::new(ServiceCollection::new())))
            }

            pub fn add<D: Into<ServiceDescriptor>>(&self, descriptor: D) {
                let mut collection = self.0.lock().unwrap();
                collection.add(descriptor);
            }

            pub fn provider(&self) -> ServiceProvider {
                let collection = self.0.lock().unwrap();
                collection.build_provider().unwrap()
            }
        }

        impl DIProvider {
            pub fn new(provider: ServiceProvider) -> Self {
                Self(Arc::new(provider))
            }
        }

        pub static DI_CONTAINER: Lazy<Arc<DIStore>> = Lazy::new(|| {
            let store = Arc::new(DIStore::new());
            #func_name(store.clone());
            store
        });

        pub static DI_PROVIDER: Lazy<Arc<DIProvider>> =
            Lazy::new(|| Arc::new(DIProvider::new(DI_CONTAINER.provider())));
    }
    .into()
}

#[derive(FromMeta)]
struct InjectParams {
    #[darling(default, rename = "as")]
    as_impl: Option<Expr>,
    #[darling(default)]
    descriptor: Option<String>,
}

pub fn inject_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return proc_macro::TokenStream::from(Error::from(e).write_errors());
        }
    };

    let InjectParams { as_impl, descriptor } = match InjectParams::from_list(&attr_args) {
        Ok(params) => params,
        Err(error) => {
            return proc_macro::TokenStream::from(Error::from(error).write_errors());
        }
    };

    let ItemStruct {
        ident,
        struct_token,
        attrs,
        vis,
        generics,
        fields,
        semi_token,
    } = parse_macro_input!(item as ItemStruct);

    let injectable_trait = as_impl.unwrap_or_else(|| parse_quote!(#ident));

    let descriptor_method = descriptor.as_deref().unwrap_or("singleton");
    let descriptor_ident = syn::Ident::new(descriptor_method, proc_macro2::Span::call_site());

    let reg_fn_name = syn::Ident::new(&format!("auto_reg_{}", ident), proc_macro2::Span::call_site());

    let expanded = quote::quote! {
        use di::*;

        #[injectable(#injectable_trait)]
        #(#attrs)*
        #vis #struct_token #ident #generics #fields #semi_token

        #[ctor::ctor]
        fn #reg_fn_name() {
            crate::DI_CONTAINER.add(#ident::#descriptor_ident());
        }
    };

    expanded.into()
}

pub fn container_get_impl(_: TokenStream) -> TokenStream {
    let getter = quote::quote!({ crate::DI_PROVIDER.0.get_required() });

    getter.into()
}

pub fn container_get_many_impl(_: TokenStream) -> TokenStream {
    let getter = quote::quote!({ crate::DI_PROVIDER.0.get_all().collect() });

    getter.into()
}
