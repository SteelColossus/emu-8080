use crate::{Register, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn inr_instruction(state: &mut State, register: Register) {
    state.increase_register(register, 1, false);
    let result = state.get_register_value(register);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
pub fn dcr_instruction(state: &mut State, register: Register) {
    state.decrease_register(register, 1, false);
    let result = state.get_register_value(register);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
pub fn add_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    adi_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn adi_instruction(state: &mut State, data: u8) {
    state.increase_register(Register::A, data, true);
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
}

#[cfg_attr(test, mutate)]
pub fn sub_instruction(state: &mut State, source_register: Register) {
    let source_register_value = state.get_register_value(source_register);
    sui_instruction(state, source_register_value);
}

#[cfg_attr(test, mutate)]
pub fn sui_instruction(state: &mut State, data: u8) {
    state.decrease_register(Register::A, data, true);
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
}
