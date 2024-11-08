// Copyright 2024 Kore Ledger
// SPDX-License-Identifier: AGPL-3.0-or-later

mod error;
mod externf;
mod value_wrapper;
use borsh::{BorshDeserialize, BorshSerialize};
use error::Error;
use json_patch::{patch, Patch};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub use self::value_wrapper::ValueWrapper;

/// Contrat execution context.
#[derive(Serialize, Deserialize, Debug)]
pub struct Context<State, Event> {
    /// Initial state of the contract
    pub initial_state: State,
    /// Event that triggered the contract execution
    pub event: Event,
    /// Is the sender of the event the owner of the contract
    pub is_owner: bool,
}

/// Contract execution result.
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractResult<State> {
    /// Final state of the contract.
    pub final_state: State,
    /// Is the contract execution successful?
    pub success: bool,
}

/// Contract init result.
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractInitCheck {
    /// Is the contract init successful?
    pub success: bool,
}

/// Internal contract execution result used for borsh serialization.
#[derive(BorshSerialize)]
struct ContractResultBorsh {
    /// Final state of the contract.
    pub final_state: ValueWrapper,
    /// Is the contract execution successful?
    pub success: bool,
}

/// Internal contract execution result implementation for errors.
impl ContractResultBorsh {
    pub fn error() -> Self {
        Self {
            final_state: ValueWrapper(serde_json::Value::Null),
            success: false,
        }
    }
}

/// Internal contract execution result used for borsh serialization.
#[derive(BorshSerialize)]
struct ContractInitCheckBorsh {
    /// Is the contract execution successful?
    pub success: bool,
}

/// Internal contract execution result implementation for errors.
impl ContractInitCheckBorsh {
    pub fn error() -> Self {
        Self {
            success: false,
        }
    }

    pub fn ok() -> Self {
        Self {
            success: true,
        }
    }
}

/// Internal contract execution result implementation for building results.
impl<State> ContractResult<State> {
    pub fn new(state: State) -> Self {
        Self {
            final_state: state,
            success: false,
        }
    }
}

/// Contract execution.
///
/// # Arguments
///
/// * `state_ptr` - Pointer to the initial state of the contract.
/// * `callback` - Callback that will be executed with the init contract logic.
///
/// # Returns
///
/// * `result_ptr` - Pointer to the init contract execution result.
///
pub fn check_init_data<State, F>(
    state_ptr: i32,
    callback: F,
) -> u32 
where
    State: for<'a> Deserialize<'a> + Serialize + Clone,
    F: Fn(&State, &mut ContractInitCheck),
{
    {
        'process: {
            let Ok(state_value) = deserialize(get_from_context(state_ptr)) else {
                break 'process;
            };
            let Ok(state) = serde_json::from_value::<State>(state_value.0) else {
                break 'process;
            };
            let mut contract_result = ContractInitCheck {success: false};
            callback(&state, &mut contract_result);

            if !contract_result.success {
                break 'process;
            }

            let Ok(result_ptr) = store(&ContractInitCheckBorsh::ok()) else {
                break 'process;
            };
            return result_ptr;
        }
        store(&ContractInitCheckBorsh::error()).expect("Contract store process failed")
    }
}

/// Contract execution.
///
/// # Arguments
///
/// * `state_ptr` - Pointer to the initial state of the contract.
/// * `event_ptr` - Pointer to the event that triggered the contract execution.
/// * `is_owner` - Is the sender of the event the owner of the contract?
/// * `callback` - Callback that will be executed with the contract logic.
///
/// # Returns
///
/// * `result_ptr` - Pointer to the contract execution result.
///
pub fn execute_contract<F, State, Event>(
    state_ptr: i32,
    event_ptr: i32,
    is_owner: i32,
    callback: F,
) -> u32
where
    State: for<'a> Deserialize<'a> + Serialize + Clone,
    Event: for<'a> Deserialize<'a> + Serialize,
    F: Fn(&Context<State, Event>, &mut ContractResult<State>),
{
    {
        'process: {
            let Ok(state_value) = deserialize(get_from_context(state_ptr)) else {
                break 'process;
            };
            let Ok(state) = serde_json::from_value::<State>(state_value.0) else {
                break 'process;
            };
            let Ok(event_value) = deserialize(get_from_context(event_ptr)) else {
                break 'process;
            };
            let Ok(event) = serde_json::from_value::<Event>(event_value.0) else {
                break 'process;
            };
            let is_owner = if is_owner == 1 { true } else { false };
            let context = Context {
                initial_state: state.clone(),
                event,
                is_owner,
            };
            let mut contract_result = ContractResult::new(state);
            callback(&context, &mut contract_result);
            let Ok(state_value) = serde_json::to_value(&contract_result.final_state) else {
                break 'process;
            };
            let result = ContractResultBorsh {
                final_state: ValueWrapper(state_value),
                success: contract_result.success,
            };
            // Después de haber sido modificado debemos guardar el nuevo estado.
            // Sería interesante no tener que guardar estado si el evento no es modificante
            let Ok(result_ptr) = store(&result) else {
                break 'process;
            };
            return result_ptr;
        };
        store(&ContractResultBorsh::error()).expect("Contract store process failed")
    }
}

fn deserialize(bytes: Vec<u8>) -> Result<ValueWrapper, Error> {
    BorshDeserialize::try_from_slice(&bytes).map_err(|_| Error::DeserializationError)
}

fn serialize<S: BorshSerialize>(data: S) -> Result<Vec<u8>, Error> {
    borsh::to_vec(&data).map_err(|_| Error::SerializationError)
}

fn get_from_context(pointer: i32) -> Vec<u8> {
    let data = unsafe {
        let len = externf::pointer_len(pointer);
        let mut data = vec![];
        for i in 0..len {
            data.push(externf::read_byte(pointer + i));
        }
        data
    };
    data
}

pub fn apply_patch<State: for<'a> Deserialize<'a> + Serialize>(
    patch_arg: Value,
    state: &State,
) -> Result<State, i32> {
    let patch_data: Patch = serde_json::from_value(patch_arg).unwrap();
    let mut state = serde_json::to_value(state).unwrap();
    patch(&mut state, &patch_data).unwrap();
    Ok(serde_json::from_value(state).unwrap())
}



fn store<S>(data: &S) -> Result<u32, Error>
where 
    S: BorshSerialize
{
    let bytes = serialize(&data).map_err(|_| Error::SerializationError)?;
    unsafe {
        let ptr = externf::alloc(bytes.len() as u32) as u32;
        for (index, byte) in bytes.into_iter().enumerate() {
            externf::write_byte(ptr, index as u32, byte);
        }
        Ok(ptr)
    }
}

#[cfg(test)]
mod tests {
}
