use crate::{
    arithmetic_instructions, branch_instructions, logical_instructions, stack_instructions,
    transfer_instructions, Operation, Register, State,
};
#[cfg(test)]
use mutagen::mutate;

#[cfg_attr(test, mutate)]
pub fn run_operation(
    operation: &Operation,
    state: &mut State,
    additional_byte_1: Option<u8>,
    additional_byte_2: Option<u8>,
) {
    let mut is_low_data_required = false;
    let mut is_high_data_required = false;

    let mut get_low_data = || {
        is_low_data_required = true;
        additional_byte_1.expect("Expected byte 1 to be present but it was not")
    };

    let mut get_high_data = || {
        is_high_data_required = true;
        additional_byte_2.expect("Expected byte 2 to be present but it was not")
    };

    match operation {
        Operation::Mov(source_register, destination_register) => {
            transfer_instructions::mov_instruction(state, *source_register, *destination_register)
        }
        Operation::MovToMem(register) => {
            transfer_instructions::mov_to_mem_instruction(state, *register)
        }
        Operation::MovFromMem(register) => {
            transfer_instructions::mov_from_mem_instruction(state, *register)
        }
        Operation::Mvi(register) => {
            transfer_instructions::mvi_instruction(state, *register, get_low_data() as i8)
        }
        Operation::MviMem => {
            transfer_instructions::mvi_mem_instruction(state, get_low_data() as i8)
        }
        Operation::Lxi(register_pair) => transfer_instructions::lxi_instruction(
            state,
            *register_pair,
            get_low_data() as i8,
            get_high_data() as i8,
        ),
        Operation::Lda => {
            transfer_instructions::lda_instruction(state, get_low_data(), get_high_data())
        }
        Operation::Sta => {
            transfer_instructions::sta_instruction(state, get_low_data(), get_high_data())
        }
        Operation::Lhld => {
            transfer_instructions::lhld_instruction(state, get_low_data(), get_high_data())
        }
        Operation::Shld => {
            transfer_instructions::shld_instruction(state, get_low_data(), get_high_data())
        }
        Operation::Ldax(register_pair) => {
            transfer_instructions::ldax_instruction(state, *register_pair)
        }
        Operation::Stax(register_pair) => {
            transfer_instructions::stax_instruction(state, *register_pair)
        }
        Operation::Xchg => transfer_instructions::xchg_instruction(state),
        Operation::Add(register) => arithmetic_instructions::add_instruction(state, *register),
        Operation::Adi => arithmetic_instructions::adi_instruction(state, get_low_data() as i8),
        Operation::Aci => arithmetic_instructions::aci_instruction(state, get_low_data() as i8),
        Operation::Sub(register) => arithmetic_instructions::sub_instruction(state, *register),
        Operation::Sui => arithmetic_instructions::sui_instruction(state, get_low_data() as i8),
        Operation::Sbi => arithmetic_instructions::sbi_instruction(state, get_low_data() as i8),
        Operation::Inr(register) => arithmetic_instructions::inr_instruction(state, *register),
        Operation::InrMem => arithmetic_instructions::inr_mem_instruction(state),
        Operation::Dcr(register) => arithmetic_instructions::dcr_instruction(state, *register),
        Operation::DcrMem => arithmetic_instructions::dcr_mem_instruction(state),
        Operation::Inx(register_pair) => {
            arithmetic_instructions::inx_instruction(state, *register_pair)
        }
        Operation::Dcx(register_pair) => {
            arithmetic_instructions::dcx_instruction(state, *register_pair)
        }
        Operation::Dad(register_pair) => {
            arithmetic_instructions::dad_instruction(state, *register_pair)
        }
        Operation::Ana(register) => logical_instructions::ana_instruction(state, *register),
        Operation::AnaMem => logical_instructions::ana_mem_instruction(state),
        Operation::Ani => logical_instructions::ani_instruction(state, get_low_data() as i8),
        Operation::Xra(register) => logical_instructions::xra_instruction(state, *register),
        Operation::XraMem => logical_instructions::xra_mem_instruction(state),
        Operation::Xri => logical_instructions::xri_instruction(state, get_low_data() as i8),
        Operation::Ora(register) => logical_instructions::ora_instruction(state, *register),
        Operation::OraMem => logical_instructions::ora_mem_instruction(state),
        Operation::Ori => logical_instructions::ori_instruction(state, get_low_data() as i8),
        Operation::Cmp(register) => logical_instructions::cmp_instruction(state, *register),
        Operation::CmpMem => logical_instructions::cmp_mem_instruction(state),
        Operation::Cpi => logical_instructions::cpi_instruction(state, get_low_data() as i8),
        Operation::Rlc => logical_instructions::rlc_instruction(state),
        Operation::Rrc => logical_instructions::rrc_instruction(state),
        Operation::Ral => logical_instructions::ral_instruction(state),
        Operation::Rar => logical_instructions::rar_instruction(state),
        Operation::Cma => logical_instructions::cma_instruction(state),
        Operation::Cmc => logical_instructions::cmc_instruction(state),
        Operation::Stc => logical_instructions::stc_instruction(state),
        Operation::Jmp => {
            branch_instructions::jmp_instruction(state, get_low_data(), get_high_data())
        }
        Operation::Jcond(condition) => branch_instructions::jcond_instruction(
            state,
            get_low_data(),
            get_high_data(),
            *condition,
        ),
        Operation::Call => {
            branch_instructions::call_instruction(state, get_low_data(), get_high_data())
        }
        Operation::Ccond(condition) => branch_instructions::ccond_instruction(
            state,
            get_low_data(),
            get_high_data(),
            *condition,
        ),
        Operation::Ret => branch_instructions::ret_instruction(state),
        Operation::Rcond(condition) => branch_instructions::rcond_instruction(state, *condition),
        Operation::Pchl => branch_instructions::pchl_instruction(state),
        Operation::Push(register_pair) => {
            stack_instructions::push_instruction(state, *register_pair)
        }
        Operation::Pop(register_pair) => stack_instructions::pop_instruction(state, *register_pair),
        Operation::PushPsw => stack_instructions::push_psw_instruction(state),
        Operation::PopPsw => stack_instructions::pop_psw_instruction(state),
        Operation::Xthl => {
            println!("-- Skipping over UNIMPLEMENTED instruction - this may cause incorrect behaviour! --");
        }
        Operation::In => {
            println!("-- Skipping over UNIMPLEMENTED instruction - this may cause incorrect behaviour! --");
            let port_number = get_low_data();
            match port_number {
                1 => {
                    println!("Port {}", port_number);
                    state.set_register(Register::A, 0b0000_1000);
                }
                2 => {
                    println!("Port {}", port_number);
                    state.set_register(Register::A, 0b0000_0000);
                }
                _ => panic!("Can't handle Port {}", port_number),
            };
        }
        Operation::Out => {
            println!("-- Skipping over UNIMPLEMENTED instruction - this may cause incorrect behaviour! --");
            let port_number = get_low_data();
            match port_number {
                3 | 5 | 6 => println!("Port {}", port_number),
                _ => panic!("Can't handle Port {}", port_number),
            };
        }
        Operation::Ei => stack_instructions::ei_instruction(state),
        Operation::Di => stack_instructions::di_instruction(state),
        Operation::Hlt => todo!(),
        Operation::Nop => (),
    };

    if !is_high_data_required && additional_byte_2.is_some() {
        panic!("Expected byte 2 to not be present but it was");
    }

    if !is_low_data_required && additional_byte_1.is_some() {
        panic!("Expected byte 1 to not be present but it was");
    }
}
