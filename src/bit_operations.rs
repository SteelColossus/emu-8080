#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn is_bit_set(value: i8, bit_index: u8) -> bool {
    if bit_index >= 8 {
        panic!("Invalid bit index of {}", bit_index);
    }

    let shifted_value = value >> bit_index;
    shifted_value & 0b00000001 != 0
}

#[cfg_attr(test, mutate)]
pub fn set_bit_in_value(value: &mut i8, bit_index: u8, bit_flag: bool) {
    if bit_index >= 8 {
        panic!("Invalid bit index of {}", bit_index);
    }

    let bit_mask = 1 << bit_index;
    let bit_value_mask = if bit_flag { bit_mask } else { 0b00000000 };
    *value = *value & !bit_mask | bit_value_mask;
}

#[cfg_attr(test, mutate)]
pub fn get_parity(value: i8) -> bool {
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
    ((value & 0x00FF) as u8, (value >> 8) as u8)
}