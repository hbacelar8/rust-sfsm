#![cfg_attr(not(feature = "std"), no_std)]

/// Trait for the state behavior.
pub trait StateBehavior {
    type State: Clone + Copy + PartialEq + Default;
    type Event: Clone + Copy + PartialEq;
    type Context: Default;

    /// Handle an event and return the next state (if a transition occurs).
    fn handle(&self, event: &Self::Event, _context: &mut Self::Context) -> Option<Self::State>;

    /// State entry.
    fn enter(&self, _context: &mut Self::Context) {}

    /// State exit.
    fn exit(&self, _context: &mut Self::Context) {}
}

/// # Rust Static FSM
///
/// A full static Rust finite state machine macro library.
///
/// ## Usage
///
/// The `rust_sfsm` macro takes as input the state machine's name, the states enum,
/// the events enum and the context struct.
///
/// The state machine's name can be just an ident if no other member is necessary
/// to the struct:
///
/// ```rust,ignore
/// use rust_sfsm::{rust_sfsm, StateBehavior};
///
/// #[derive(Clone, Copy, PartialEq)]
/// enum States {
///     (...)
/// }
///
/// #[derive(Clone, Copy, PartialEq)]
/// enum Events {
///     (...)
/// }
///
/// #[derive(Default)]
/// struct Context {
///     (...)
/// }
///
/// rust_sfsm!(FooName, States, Events, Context);
/// ```
///
/// The state machine's name can also be a struct with default values if data
/// other than the cotext is desired:
///
/// ```rust,ignore
/// rust_sfsm!(
///     FooName {
///         foo_data: u16 = 0,
///         boo_data: boo = false,
///     },
///     States,
///     Events,
///     Context
/// );
/// ```
#[macro_export]
macro_rules! rust_sfsm {
    // Case 1: With additional members for the state machine struct
    (
        $state_machine_name:ident {
            $($member_field:ident: $member_field_type:ty = $member_default:expr),* $(,)?
        },
        $state_type:ident,
        $event_type:ident,
        $context_type:ident
    ) => {
        rust_sfsm!(@generate $state_machine_name, $state_type, $event_type, $context_type,
            members { $($member_field: $member_field_type = $member_default),* }
        );
    };

    // Case 2: Without additional members for the state machine struct
    (
        $state_machine_name:ident,
        $state_type:ident,
        $event_type:ident,
        $context_type:ident
    ) => {
        rust_sfsm!(@generate $state_machine_name, $state_type, $event_type, $context_type,
            members { }
        );
    };

    // Internal implementation for generating the state machine
    (
        @generate $state_machine_name:ident, $state_type:ident, $event_type:ident, $context_type:ident,
        members { $($member_field:ident: $member_field_type:ty = $member_default:expr),* }
    ) => {
        /// State machine struct.
        pub struct $state_machine_name {
            current_state: $state_type,
            context: $context_type,
            $(
                $member_field: $member_field_type,
            )*
        }

        impl $state_machine_name {
            /// Create a new state machine.
            pub fn new() -> Self {
                Self {
                    current_state: Default::default(),
                    context: Default::default(),
                    $(
                        $member_field: $member_default,
                    )*
                }
            }

            /// Transit to a new state.
            pub fn transit(&mut self, new_state: $state_type) {
                self.current_state.exit(&mut self.context);
                self.current_state = new_state;
                self.current_state.enter(&mut self.context);
            }

            /// Force transition to a new state without calls to respectives
            /// `enter` and `exit` functions.
            pub fn force_state(&mut self, new_state: $state_type) {
                self.current_state = new_state;
            }

            /// Get the current state
            pub fn current_state(&self) -> $state_type {
                self.current_state
            }

            /// Handle event and transition if necessary.
            fn handle(&mut self, event: $event_type) {
                if let Some(next_state) = self.current_state.handle(&event, &mut self.context) {
                    self.current_state.exit(&mut self.context);
                    self.current_state = next_state;
                    self.current_state.enter(&mut self.context);
                }
            }
        }
    };
}
