use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_macro(&ast)
}

fn impl_derive_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Component for #name {
            fn inner_type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }
            fn as_any(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
    gen.into()
}
