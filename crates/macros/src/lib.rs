use proc_macro::TokenStream;

use crate::di::{
    container_get_impl, container_get_many_impl, initialize_container_impl, inject_impl,
};

extern crate proc_macro;
mod di;

#[proc_macro]
pub fn initialize_container(item: TokenStream) -> TokenStream {
    initialize_container_impl(item)
}

#[proc_macro_attribute]
pub fn inject(args: TokenStream, item: TokenStream) -> TokenStream {
    inject_impl(args, item)
}

/// a macro to get a single item from the DI container
/// for impl Trait services it returns the last injected implementation.
/// To get all impl Trait services use `container_get_many!()` which returns an iterator
/// Example:
/// * let a_foo: Arc<dyn FooTrait> = container_get!();
/// * let the_foo = Arc<FooStruct> = container_get!();
#[proc_macro]
pub fn container_get(item: TokenStream) -> TokenStream {
    container_get_impl(item)
}

/// a macro to get a all item injected as a Trait type in the DI container
/// To get a single impl Trait services use `container_get!()` which returns the last injected implementation.
/// Example:
/// * let foos: Vec<Arc<dyn Foo>> = container_get_many!();
#[proc_macro]
pub fn container_get_many(item: TokenStream) -> TokenStream {
    container_get_many_impl(item)
}
