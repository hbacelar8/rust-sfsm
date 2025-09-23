use rust_sfsm::{StateBehavior, rust_sfsm};

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
#[derive(Clone, Copy, PartialEq)]
enum Events {
    Create,
    Open,
    Close,
    Lock,
    Unlock,
}

/// Protocol state machine context.
#[derive(Default)]
struct Context {
    lock_counter: u16,
}

impl StateBehavior for States {
    type State = States;
    type Event = Events;
    type Context = Context;

    fn enter(&self, _context: &mut Self::Context) {
        if self == &States::Locked {
            _context.lock_counter += 1
        }
    }

    fn handle(&self, event: &Self::Event, _context: &mut Self::Context) -> Option<Self::State> {
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

impl Protocol {
    /// Get number of protocol locking operations.
    fn lock_counter(&self) -> u16 {
        self.context.lock_counter
    }
}

rust_sfsm!(Protocol, States, Events, Context);

fn main() {
    let mut protocol = Protocol::new();

    assert!(protocol.current_state() == States::Init);

    protocol.handle(Events::Create);
    assert!(protocol.current_state() == States::Opened);

    protocol.handle(Events::Close);
    assert!(protocol.current_state() == States::Closed);

    protocol.handle(Events::Lock);
    assert!(protocol.current_state() == States::Locked);
    assert!(protocol.lock_counter() == 1);

    protocol.handle(Events::Unlock);
    assert!(protocol.current_state() == States::Closed);

    protocol.handle(Events::Open);
    assert!(protocol.current_state() == States::Opened);
}
