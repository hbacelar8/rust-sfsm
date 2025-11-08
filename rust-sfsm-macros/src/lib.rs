use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Field, FieldMutability, Fields, FieldsNamed, Ident, Item, ItemStruct, Path, Type, TypePath,
    Visibility, parse, parse_macro_input, token::Colon,
};

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Args {
    states: Path,
    context: Path,
}

/// # Rust-SFSM Attribute Macro.
///
/// SFSM stands for Static Finite State Machine.
///
/// This macro must be used on `struct`'s and implements
/// the boilerplate for any type that has a state-like behavior.
///
/// ## Example
///
/// ```rust
/// use rust_sfsm::{StateBehavior, StateMachine, rust_sfsm};
///
/// /// List of protocol states.
/// #[derive(Clone, Copy, Default, PartialEq)]
/// enum States {
///     #[default]
///     Init,
///     Opened,
///     Closed,
///     Locked,
/// }
///
/// /// List of protocol events.
/// enum Events {
///     Create,
///     Open,
///     Close,
///     Lock,
///     Unlock,
/// }
///
/// /// Protocol state machine context (data shared between states).
/// #[derive(Default)]
/// struct Context {
///     lock_counter: u16,
/// }
///
/// impl StateBehavior for States {
///     type State = Self;
///     type Event<'a> = Events;
///     type Context = Context;
///
///     fn enter(&self, _context: &mut Self::Context) {
///         if self == &States::Locked {
///             _context.lock_counter += 1
///         }
///     }
///
///     fn handle_event(
///         &self,
///         event: &Self::Event<'_>,
///         _context: &mut Self::Context,
///     ) -> Option<Self::State> {
///         match (self, event) {
///             (&States::Init, &Events::Create) => Some(States::Opened),
///             (&States::Opened, &Events::Close) => Some(States::Closed),
///             (&States::Closed, &Events::Open) => Some(States::Opened),
///             (&States::Closed, &Events::Lock) => Some(States::Locked),
///             (&States::Locked, &Events::Unlock) => Some(States::Closed),
///             _ => None,
///         }
///     }
/// }
///
/// #[rust_sfsm(states = States, context = Context)]
/// struct Protocol {}
///
/// impl Protocol {
///     fn new() -> Self {
///         Self {
///             current_state: Default::default(),
///             context: Default::default(),
///         }
///     }
/// }
/// ```
///
/// ## Macro Expansion
///
/// The `rust_sfsm` macro expands to this:
///
/// ```rust
/// struct Protocol {
///     current_state: States,
///     context: Context,
/// }
///
/// impl ::rust_sfsm::StateMachine<States> for Protocol {
///     fn current_state(&self) -> <States as ::rust_sfsm::StateBehavior>::State {
///         self.current_state
///     }
///
///     fn handle_event(
///         &mut self,
///         event: &<States as ::rust_sfsm::StateBehavior>::Event<'_>,
///     ) {
///         if let Some(next_state) = self
///             .current_state
///             .handle_event(event, &mut self.context)
///         {
///             self.transit(next_state)
///         }
///     }
///
///     fn transit(&mut self, new_state: <States as ::rust_sfsm::StateBehavior>::State) {
///         self.current_state.exit(&mut self.context);
///         self.current_state = new_state;
///         self.current_state.enter(&mut self.context);
///     }
///
///     fn force_state(&mut self, new_state: <States as ::rust_sfsm::StateBehavior>::State) {
///         self.current_state = new_state;
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn rust_sfsm(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: Args = match parse(args) {
        Ok(args) => args,
        Err(e) => {
            return e.into_compile_error().into();
        }
    };

    let input = parse_macro_input!(input as Item);

    let output = match input {
        Item::Struct(mut item_struct) => {
            // add fields
            add_fields(&mut item_struct, &args);

            // add state machine impl
            let struct_ident = &item_struct.ident;
            let trait_impl = generate_state_machine_impl(struct_ident, &args);

            quote! {
                #item_struct
                #trait_impl
            }
        }

        _ => {
            return syn::Error::new_spanned(input, "rust_sfsm macro can only be applied to struct")
                .into_compile_error()
                .into();
        }
    };

    output.into()
}

fn add_fields(item_struct: &mut ItemStruct, args: &Args) {
    if let Fields::Named(FieldsNamed { named, .. }) = &mut item_struct.fields {
        let current_state_field = Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(Ident::new("current_state", proc_macro2::Span::call_site())),
            colon_token: Some(Colon::default()),
            ty: Type::Path(TypePath {
                qself: None,
                path: args.states.clone(),
            }),
        };

        let context_field = syn::Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: syn::FieldMutability::None,
            ident: Some(syn::Ident::new("context", proc_macro2::Span::call_site())),
            colon_token: Some(syn::token::Colon::default()),
            ty: syn::Type::Path(syn::TypePath {
                qself: None,
                path: args.context.clone(),
            }),
        };

        named.push(current_state_field);
        named.push(context_field);
    }
}

fn generate_state_machine_impl(struct_ident: &Ident, args: &Args) -> proc_macro2::TokenStream {
    let states_type = &args.states;

    quote! {
        impl ::rust_sfsm::StateMachine<#states_type> for #struct_ident
        {
            fn current_state(&self) -> <#states_type as ::rust_sfsm::StateBehavior>::State {
                self.current_state
            }

            fn handle_event(&mut self, event: &<#states_type as ::rust_sfsm::StateBehavior>::Event<'_>) {
                if let Some(next_state) = self.current_state.handle_event(event, &mut self.context) {
                    self.transit(next_state)
                }
            }

            fn transit(&mut self, new_state: <#states_type as ::rust_sfsm::StateBehavior>::State) {
                self.current_state.exit(&mut self.context);
                self.current_state = new_state;
                self.current_state.enter(&mut self.context);
            }

            fn force_state(&mut self, new_state: <#states_type as ::rust_sfsm::StateBehavior>::State) {
                self.current_state = new_state;
            }
        }
    }
}
