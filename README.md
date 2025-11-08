# Rust Static FSM

A full static Rust finite state machine macro library.

Compatible with `no_std` and embedded environments.

---

## Example

```rust
use rust_sfsm::{StateBehavior, StateMachine, rust_sfsm};

/// List of protocol states.
#[derive(Clone, Copy, Default, PartialEq)]
enum States {
    #[default]
    Init,
    Opened,
    Closed,
    Locked,
}

/// List of protocol events.
enum Events {
    Create,
    Open,
    Close,
    Lock,
    Unlock,
}

/// Protocol state machine context (data shared between states).
#[derive(Default)]
struct Context {
    lock_counter: u16,
}

impl StateBehavior for States {
    type State = Self;
    type Event<'a> = Events;
    type Context = Context;

    fn enter(&self, _context: &mut Self::Context) {
        if self == &States::Locked {
            _context.lock_counter += 1
        }
    }

    fn handle_event(
        &self,
        event: &Self::Event<'_>,
        _context: &mut Self::Context,
    ) -> Option<Self::State> {
        match (self, event) {
            (&States::Init, &Events::Create) => Some(States::Opened),
            (&States::Opened, &Events::Close) => Some(States::Closed),
            (&States::Closed, &Events::Open) => Some(States::Opened),
            (&States::Closed, &Events::Lock) => Some(States::Locked),
            (&States::Locked, &Events::Unlock) => Some(States::Closed),
            _ => None,
        }
    }
}

#[rust_sfsm(states = States, context = Context)]
struct Protocol {}

impl Protocol {
    fn new() -> Self {
        Self {
            current_state: Default::default(),
            context: Default::default(),
        }
    }
}

fn main() {
    let mut protocol = Protocol::new();

    test_state_machine(&mut protocol);
}

fn test_state_machine<S: StateMachine<States>>(state_machine: &mut S) {
    assert!(state_machine.current_state() == States::Init);

    state_machine.handle_event(&Events::Create);
    assert!(state_machine.current_state() == States::Opened);

    state_machine.handle_event(&Events::Close);
    assert!(state_machine.current_state() == States::Closed);

    state_machine.handle_event(&Events::Lock);
    assert!(state_machine.current_state() == States::Locked);

    state_machine.handle_event(&Events::Unlock);
    assert!(state_machine.current_state() == States::Closed);

    state_machine.handle_event(&Events::Open);
    assert!(state_machine.current_state() == States::Opened);
}
```
