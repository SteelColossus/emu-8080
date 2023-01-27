#![feature(test)]
extern crate test;
use emu_8080::{runner, Register, State, StateBuilder};
use maplit::hashmap;
use test::Bencher;

#[bench]
fn bench_create_state(b: &mut Bencher) {
    b.iter(State::default);
}

#[bench]
fn bench_fill_memory(b: &mut Bencher) {
    let mut state = StateBuilder::default()
        .register_values(hashmap! { Register::L => 6 })
        .build();
    state.load_memory(&[
        0x05, // DCR B
        0x70, // MOV M,B
        0x23, // INX HL
        0xC3, 0x00, 0x00, // JMP 0000
    ]);

    b.iter(|| {
        for _ in 0..4 {
            runner::run_next_operation(&mut state);
        }
    });
}
