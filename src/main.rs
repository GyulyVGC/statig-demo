use statig::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;

#[derive(Default)]
pub struct MyProgram {
    pub numbers: Vec<u32>,
}

#[derive(Debug)]
pub enum Event {
    NumberReceived(u32),
    NumberProcessed,
    NumberStored,
}

#[state_machine(
    initial = "State::waiting()",
    on_dispatch = "Self::on_dispatch",
    on_transition = "Self::on_transition",
    state(derive(Debug)),
    superstate(derive(Debug))
)]
impl MyProgram {
    // called before an event is dispatched to a specific state or superstate
    fn on_dispatch(&mut self, state: StateOrSuperstate<MyProgram>, event: &Event) {
        match state {
            StateOrSuperstate::State(state) => {
                println!("--- Dispatched event {event:?} to state {state:?}")
            }
            StateOrSuperstate::Superstate(superstate) => {
                println!("--- Dispatched event {event:?} to superstate {superstate:?}")
            }
        }
    }

    // called after a state transition
    fn on_transition(&mut self, from: &State, to: &State) {
        println!("--- Transitioned from {from:?} to {to:?}");
    }

    #[state]
    fn waiting(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::NumberReceived(n) => {
                self.numbers.push(*n);
                Transition(State::processing_number())
            }
            _ => Super,
        }
    }

    #[state(superstate = "busy")]
    fn processing_number(event: &Event) -> Response<State> {
        match event {
            Event::NumberProcessed => Transition(State::storing_number()),
            _ => Super,
        }
    }

    #[state(superstate = "busy")]
    fn storing_number(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::NumberStored => {
                self.numbers.remove(0);
                if self.numbers.is_empty() {
                    Transition(State::waiting())
                } else {
                    Transition(State::processing_number())
                }
            }
            _ => Super,
        }
    }

    #[superstate]
    fn busy(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::NumberReceived(n) => {
                self.numbers.push(*n);
                Handled
            }
            _ => Super,
        }
    }
}

fn main() {
    let state_machine = MyProgram::default().state_machine();

    let sm_1 = Arc::new(Mutex::new(state_machine));
    let sm_2 = sm_1.clone();

    sm_1.lock().unwrap().handle(&Event::NumberReceived(4));
    sm_1.lock().unwrap().handle(&Event::NumberReceived(7));
    sm_1.lock().unwrap().handle(&Event::NumberReceived(9));

    sleep(std::time::Duration::from_secs(2));

    let j = thread::spawn(move || do_stuff(sm_2));

    j.join().unwrap();
}

fn do_stuff(state_machine: Arc<Mutex<StateMachine<MyProgram>>>) {
    let mut my_numbers = Vec::new();
    loop {
        let num = state_machine.lock().unwrap().numbers.get(0).cloned();
        if let Some(n) = num {
            print!("Processing number {n}... ");
            let x = n.pow(2);
            println!("{n}^2 = {x}");
            state_machine
                .lock()
                .unwrap()
                .handle(&Event::NumberProcessed);
            print!("Storing number: {n}... ");
            my_numbers.push(n);
            println!("collection is now {my_numbers:?}");
            state_machine.lock().unwrap().handle(&Event::NumberStored);
        } else {
            break;
        };
    }
}
