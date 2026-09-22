use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn godot_stage_export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let plain_name = &input.ident;
    
    // Create a unique local wrapper name, e.g., CellGeneratorStageWrapper
    let wrapper_name = format_ident!("{}StageWrapper", plain_name);

    let expanded = quote! {
        #input

        // 1. Create a local "Newtype" wrapper around the foreign Gd<T> pointer
        #[doc(hidden)]
        pub struct #wrapper_name(::godot::prelude::Gd<#plain_name>);

        // 2. Implement the foreign trait on your newly created local wrapper
        impl terrain_gen_core::stage::Stage for #wrapper_name {
            fn run(&self, planet: &mut terrain_gen_core::data::PlanetData) -> anyhow::Result<()> {
                // self.0 accesses the inner Gd<T> pointer
                self.0.bind().run(planet)
            }

            fn name(&self) -> &'static str{
                self.0.bind().name()
            }
        }

        ::inventory::submit! {
            terrain_gen_core::stage::StageConverter(|res| {
                match res.try_cast::<#plain_name>() {
                    Ok(result) => {
                        // 3. Box the wrapper instead of the raw Gd<T>
                        let boxed: Box<dyn terrain_gen_core::stage::Stage> = Box::new(#wrapper_name(result));
                        Some(boxed)
                    }
                    Err(_) => None,
                }
            })
        }
    };

    expanded.into()
}