use crate::{Register, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn ana_instruction(state: &mut State, source_register: Register) {
    state.set_register_by_function_with_register(
        source_register,
        Register::A,
        |source_value, target_value| source_value & target_value,
    );
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = false;
}

#[cfg_attr(test, mutate)]
pub fn ani_instruction(state: &mut State, data: u8) {
    state.set_register_by_function_with_value(Register::A, data, |source_value, target_value| {
        source_value & target_value
    });
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = false;
}

#[cfg_attr(test, mutate)]
pub fn ora_instruction(state: &mut State, source_register: Register) {
    state.set_register_by_function_with_register(
        source_register,
        Register::A,
        |source_value, target_value| source_value | target_value,
    );
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = false;
}

#[cfg_attr(test, mutate)]
pub fn oni_instruction(state: &mut State, data: u8) {
    state.set_register_by_function_with_value(Register::A, data, |source_value, target_value| {
        source_value | target_value
    });
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = false;
}

#[cfg_attr(test, mutate)]
pub fn xra_instruction(state: &mut State, source_register: Register) {
    state.set_register_by_function_with_register(
        source_register,
        Register::A,
        |source_value, target_value| source_value ^ target_value,
    );
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = false;
}

#[cfg_attr(test, mutate)]
pub fn xni_instruction(state: &mut State, data: u8) {
    state.set_register_by_function_with_value(Register::A, data, |source_value, target_value| {
        source_value ^ target_value
    });
    let result = state.get_register_value(Register::A);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = false;
}

#[cfg_attr(test, mutate)]
pub fn cmp_instruction(state: &mut State, register: Register) {
    let accumulator_value = state.get_register_value(Register::A);
    let register_value = state.get_register_value(register);
    let (result, borrow) = accumulator_value.overflowing_sub(register_value);
    state.set_condition_flags_based_on_result(result);
    state.condition_flags.carry = borrow;
}

#[cfg_attr(test, mutate)]
pub fn cpi_instruction(state: &mut State, data: u8) {
    // This seems wrong but from the docs looks to have the same behaviour
    crate::arithmetic_instructions::sui_instruction(state, data);
}

#[cfg_attr(test, mutate)]
pub fn rlc_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    state.set_register(Register::A, accumulator_value.rotate_left(1));
    state.condition_flags.carry = state.is_bit_set(accumulator_value, 7);
}

#[cfg_attr(test, mutate)]
pub fn rrc_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    state.set_register(Register::A, accumulator_value.rotate_right(1));
    state.condition_flags.carry = state.is_bit_set(accumulator_value, 0);
}

#[cfg_attr(test, mutate)]
pub fn cma_instruction(state: &mut State) {
    let accumulator_value = state.get_register_value(Register::A);
    state.set_register(Register::A, !accumulator_value);
}

#[cfg_attr(test, mutate)]
pub fn cmc_instruction(state: &mut State) {
    state.condition_flags.carry = !state.condition_flags.carry;
}

#[cfg_attr(test, mutate)]
pub fn stc_instruction(state: &mut State) {
    state.condition_flags.carry = true;
}
