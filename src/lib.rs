#![cfg_attr(not(feature = "std"), no_std)]

pub use rust_sfsm_macros::rust_sfsm;

/// Trait for the state behavior.
///
/// ```rust
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
/// ```
pub trait StateBehavior {
    type State: Clone + Copy + PartialEq + Default;
    type Event<'a>
    where
        Self: 'a;
    type Context;

    /// Handle an event and return the next state (if a transition occurs).
    fn handle_event(
        &self,
        event: &Self::Event<'_>,
        _context: &mut Self::Context,
    ) -> Option<Self::State>;

    /// State entry.
    fn enter(&self, _context: &mut Self::Context) {}

    /// State exit.
    fn exit(&self, _context: &mut Self::Context) {}
}

/// Trait for the state machine behavior.
///
/// This trait is implemented by the [rust_sfsm] attribute macro
/// and shouldn't be manually implemented by the user.
///
/// It may be used to monomorphize different types implementing
/// the state machine behavior for a given set of states:
///
/// ```rust
/// fn test_state_machine<S: StateMachine<States>>(state_machine: &mut S) {
///     assert!(state_machine.current_state() == States::Init);
///
///     state_machine.handle_event(&Events::Create);
///     assert!(state_machine.current_state() == States::Opened);
///
///     state_machine.handle_event(&Events::Close);
///     assert!(state_machine.current_state() == States::Closed);
///
///     state_machine.handle_event(&Events::Lock);
///     assert!(state_machine.current_state() == States::Locked);
///
///     state_machine.handle_event(&Events::Unlock);
///     assert!(state_machine.current_state() == States::Closed);
///
///     state_machine.handle_event(&Events::Open);
///     assert!(state_machine.current_state() == States::Opened);
/// }
/// ```
pub trait StateMachine<S: StateBehavior> {
    /// Get the current state.
    fn current_state(&self) -> S::State;

    /// Handle an event and transit if necessary.
    fn handle_event(&mut self, event: &S::Event<'_>);

    /// Transit to a new state.
    fn transit(&mut self, new_state: S::State);

    /// Force transition to a new state without calls to respectives
    /// `enter` and `exit` functions.
    fn force_state(&mut self, new_state: S::State);
}
