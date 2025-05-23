use serde::{Serialize, Deserialize};
use kore_contract_sdk as sdk;

/// Define the state of the contract. 
#[derive(Serialize, Deserialize, Clone)]
struct State {
  pub one: u32,
  pub two: u32,
  pub three: u32
}

#[derive(Serialize, Deserialize)]
enum StateEvent {
  ModOne { data: u32 },
  ModTwo { data: u32 },
  ModThree { data: u32 },
  ModAll { one: u32, two: u32, three: u32 }
}

#[unsafe(no_mangle)]
pub unsafe fn main_function(state_ptr: i32, init_state_ptr: i32, event_ptr: i32, is_owner: i32) -> u32 {
  sdk::execute_contract(state_ptr, init_state_ptr, event_ptr, is_owner, contract_logic)
}

#[unsafe(no_mangle)]
pub unsafe fn init_check_function(state_ptr: i32) -> u32 {
  sdk::check_init_data(state_ptr, init_logic)
}

fn init_logic(
  _state: &State,
  contract_result: &mut sdk::ContractInitCheck,
) {
  contract_result.success = true;
}

fn contract_logic(
  context: &sdk::Context<State, StateEvent>,
  contract_result: &mut sdk::ContractResult<State>,
) {
  let state = &mut contract_result.final_state;
  match context.event {
      StateEvent::ModOne { data } => {
        state.one = data;
      },
      StateEvent::ModTwo { data } => {
        state.two = data;
      },
      StateEvent::ModThree { data } => {
        if data == 50 {
          contract_result.error = "Can not change three value, 50 is a invalid value".to_owned();
          return
        }
        
        state.three = data;
      },
      StateEvent::ModAll { one, two, three } => {
        state.one = one;
        state.two = two;
        state.three = three;
      }
  }
  contract_result.success = true;
}

#[test]
fn contract_test() {
  let initial_state = State {
    one: 1,
    two: 2,
    three: 3
  };
  let context = sdk::Context {
    initial_state: initial_state.clone(),
    event: StateEvent::ModOne { data: 100 },
    is_owner: false
  };
  let mut result = sdk::ContractResult::new(initial_state);
  contract_logic(&context, &mut result);
  assert_eq!(result.final_state.one, 100);
  assert!(result.success);
}

#[test]
fn contract_test_fail() {
  let initial_state = State {
    one: 1,
    two: 2,
    three: 3
  };
  let context = sdk::Context {
    initial_state: initial_state.clone(),
    event: StateEvent::ModThree { data: 50 },
    is_owner: false
  };
  let mut result = sdk::ContractResult::new(initial_state);
  contract_logic(&context, &mut result);
  assert_eq!(result.final_state.three, 3);
  assert_eq!(result.error, "Can not change three value, 50 is a invalid value");
  assert!(!result.success);
}
