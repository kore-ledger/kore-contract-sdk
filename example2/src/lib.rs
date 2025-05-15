use serde::{Serialize, Deserialize};
use kore_contract_sdk as sdk;

/// Define the state of the contract. 
#[derive(Serialize, Deserialize, Clone)]
struct State {
  pub data: String
}

#[derive(Serialize, Deserialize, Clone)]
enum StateEvent {
  ChangeData { data: String },
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
  match context.event.clone() {
      StateEvent::ChangeData { data } => {
        state.data = data.clone();
      }
  }
  contract_result.success = true;
}

#[test]
fn contract_test() {
  let initial_state = State {
    data: "".to_owned()
  };

  let context = sdk::Context {
    initial_state: initial_state.clone(),
    event: StateEvent::ChangeData { data: "KoreLedger".to_owned() },
    is_owner: false
  };
  let mut result = sdk::ContractResult::new(initial_state);
  contract_logic(&context, &mut result);
  assert_eq!(result.final_state.data, "KoreLedger");
  assert!(result.success);
}
