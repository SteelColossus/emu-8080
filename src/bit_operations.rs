#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn is_bit_set(value: u8, bit_index: u8) -> bool {
    if bit_index >= 8 {
        panic!("Invalid bit index of {}", bit_index);
    }

    let shifted_value = value >> bit_index;
    shifted_value & 0b00000001 != 0
}

#[cfg_attr(test, mutate)]
pub fn set_bit_in_value(value: &mut u8, bit_index: u8, bit_flag: bool) {
    if bit_index >= 8 {
        panic!("Invalid bit index of {}", bit_index);
    }

    let bit_mask = 1 << bit_index;
    let bit_value_mask = if bit_flag { bit_mask } else { 0b00000000 };
    *value = *value & !bit_mask | bit_value_mask;
}

#[cfg_attr(test, mutate)]
pub fn get_parity(value: u8) -> bool {
    let mut parity = true;

    for bit_index in 0..=7 {
        if is_bit_set(value, bit_index) {
            parity = !parity
        }
    }

    parity
}

#[cfg_attr(test, mutate)]
pub fn concat_low_high_bytes(low_byte: u8, high_byte: u8) -> u16 {
    u16::from(high_byte) << 8 | u16::from(low_byte)
}

#[cfg_attr(test, mutate)]
pub fn split_to_low_high_bytes(value: u16) -> (u8, u8) {
    ((value & 0b0000_0000_1111_1111) as u8, (value >> 8) as u8)
}

#[cfg_attr(test, mutate)]
pub fn calculate_auxiliary_carry(value_1: u8, value_2: u8, is_subtraction: bool) -> bool {
    let nibble_mask = 0b0000_1111;
    let lower_value_1 = value_1 & nibble_mask;
    let lower_value_2 = value_2 & nibble_mask;
    let result = if is_subtraction {
        lower_value_1.wrapping_sub(lower_value_2)
    } else {
        lower_value_1.wrapping_add(lower_value_2)
    };
    let is_bit_set = is_bit_set(result, 4);
    // Subtraction for the 8080 is implemented via unsigned addition, i.e. where
    // the second argument is converted to its inverse and then added instead of subtracted.
    // For the carry flag, the resulting flag is then inverted if it is a subtraction
    // to represent a borrow instead of a carry.
    // However, no such behaviour exists for the auxiliary carry flag -
    // the flag is always treated as if it is an addition.
    // Counter-intuitively, this means we actually need to account for this behaviour with our approach,
    // inverting the flag if is a subtraction to simulate the (incorrect) behaviour.
    // See https://retrocomputing.stackexchange.com/questions/12558/the-behavior-of-the-auxiliary-carry-flag-in-subtraction-on-intel-8080.
    if is_subtraction {
        !is_bit_set
    } else {
        is_bit_set
    }
}

#[cfg_attr(test, mutate)]
pub fn reverse_byte(value: u8) -> u8 {
    let mut reverse_bit_mask = 0b1000_0000;
    let mut reverse_byte = 0;

    for bit_index in 0..=7 {
        if is_bit_set(value, bit_index) {
            reverse_byte += reverse_bit_mask;
        }
        reverse_bit_mask >>= 1;
    }

    reverse_byte
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Invalid bit index of 8")]
    fn is_bit_set_panics_when_given_an_invalid_bit_index() {
        is_bit_set(127, 8);
    }

    #[test]
    #[should_panic(expected = "Invalid bit index of 8")]
    fn get_value_with_bit_set_panics_when_given_an_invalid_bit_index() {
        set_bit_in_value(&mut 127, 8, true);
    }
}
