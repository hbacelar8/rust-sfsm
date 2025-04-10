use rustfsm::{StateBehavior, rustfsm};

#[derive(Clone, Copy, PartialEq)]
enum MarioConsumables {
    Mushroom,
    Flower,
    Feather,
}

#[derive(Clone, Copy, PartialEq)]
enum AliveStates {
    SmallMario,
    BigMario(BigMarioStates),
}

#[derive(Clone, Copy, PartialEq)]
#[allow(clippy::enum_variant_names)]
enum BigMarioStates {
    SuperMario,
    CapeMario,
    FireMario,
}

#[derive(Clone, Copy, PartialEq)]
enum States {
    AliveMario(AliveStates),
    DeadMario,
}

#[derive(Clone, Copy, PartialEq)]
enum Events {
    GetConsumable(MarioConsumables),
    GetHit,
}

struct Context {
    number_of_lifes: u8,
    number_of_coins: u16,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            number_of_lifes: 1,
            number_of_coins: 0,
        }
    }
}

// Generate the state machine
rustfsm!(Mario, States, Events, Context);

impl StateBehavior for States {
    type State = States;
    type Event = Events;
    type Context = Context;

    fn enter(&self, context: &mut Self::Context) {
        match self {
            States::AliveMario(AliveStates::SmallMario) => context.number_of_coins = 0,

            States::AliveMario(AliveStates::BigMario(BigMarioStates::SuperMario)) => {
                context.number_of_coins += 100
            }

            States::AliveMario(AliveStates::BigMario(BigMarioStates::FireMario)) => {
                context.number_of_coins += 200
            }

            States::AliveMario(AliveStates::BigMario(BigMarioStates::CapeMario)) => {
                context.number_of_coins += 300
            }

            States::DeadMario => context.number_of_lifes = 0,
        }
    }

    fn handle(&self, event: &Self::Event, context: &mut Self::Context) -> Option<Self::State> {
        use AliveStates::*;
        use BigMarioStates::*;
        use Events::*;
        use MarioConsumables::*;
        use States::*;

        match (self, event) {
            (AliveMario(SmallMario), GetConsumable(Mushroom)) => {
                Some(AliveMario(BigMario(SuperMario)))
            }

            (
                AliveMario(SmallMario)
                | AliveMario(BigMario(SuperMario))
                | AliveMario(BigMario(CapeMario)),
                GetConsumable(Flower),
            ) => Some(AliveMario(BigMario(FireMario))),

            (
                AliveMario(SmallMario)
                | AliveMario(BigMario(SuperMario))
                | AliveMario(BigMario(FireMario)),
                GetConsumable(Feather),
            ) => Some(AliveMario(BigMario(CapeMario))),

            (AliveMario(SmallMario), GetHit) => {
                if context.number_of_lifes == 1 {
                    Some(DeadMario)
                } else {
                    context.number_of_lifes -= 1;
                    None
                }
            }

            (AliveMario(BigMario(_)), GetHit) => Some(AliveMario(SmallMario)),

            _ => None,
        }
    }
}

impl Mario {
    fn is_alive(&self) -> bool {
        self.current_state != States::DeadMario
    }

    fn get_number_of_lifes(&self) -> u8 {
        self.context.number_of_lifes
    }

    fn get_number_of_coins(&self) -> u16 {
        self.context.number_of_coins
    }

    fn get_consummable(&mut self, consummable: MarioConsumables) {
        self.handle(Events::GetConsumable(consummable));
    }

    fn get_hit(&mut self) {
        self.handle(Events::GetHit);
    }
}

fn main() {
    let mut mario = Mario::new(States::AliveMario(AliveStates::SmallMario));

    // Get a mushroom
    mario.get_consummable(MarioConsumables::Mushroom);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::SuperMario)),
    );
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 100);
    assert!(mario.is_alive());

    // Get a hit
    mario.get_hit();
    assert!(mario.current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 0);
    assert!(mario.is_alive());

    // Get a flower
    mario.get_consummable(MarioConsumables::Flower);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::FireMario))
    );
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 200);
    assert!(mario.is_alive());

    // Get a feather
    mario.get_consummable(MarioConsumables::Feather);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::CapeMario))
    );
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 500);
    assert!(mario.is_alive());

    // Get a hit
    mario.get_hit();
    assert!(mario.current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 0);
    assert!(mario.is_alive());

    // Oh no
    mario.get_hit();
    assert!(mario.current_state() == States::DeadMario);
    assert!(mario.get_number_of_lifes() == 0);
    assert!(mario.get_number_of_coins() == 0);
    assert!(!mario.is_alive());
}
