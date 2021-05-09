use crate::{Register, State};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn mvi_instruction(state: &mut State, register: Register, data: u8) {
    state.set_register(register, data);
}

#[cfg_attr(test, mutate)]
pub fn mov_instruction(state: &mut State, from_register: Register, to_register: Register) {
    let from_register_value = state.get_register_value(from_register);
    mvi_instruction(state, to_register, from_register_value);
}

#[cfg_attr(test, mutate)]
pub fn xchg_instruction(state: &mut State) {
    state.exchange_register_values(Register::D, Register::H);
    state.exchange_register_values(Register::E, Register::L);
}
