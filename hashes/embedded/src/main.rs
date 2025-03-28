#![cfg_attr(feature = "alloc", feature(alloc_error_handler))]
#![no_std]
#![no_main]

#[macro_use]
extern crate bitcoin_hashes;

#[cfg(feature = "alloc")] extern crate alloc;
#[cfg(feature = "alloc")] use alloc_cortex_m::CortexMHeap;
#[cfg(feature = "alloc")] use alloc::string::ToString;

use bitcoin_hashes::{sha256, HashEngine};
use bitcoin_io::Write;
use cortex_m_rt::entry;
use cortex_m_semihosting::debug;
#[cfg(feature = "hex")]
use cortex_m_semihosting::hprintln;
use panic_halt as _;

hash_newtype! {
    struct TestType(sha256::Hash);
}

#[cfg(feature = "hex")]
bitcoin_hashes::impl_hex_for_newtype!(TestType);
#[cfg(not(feature = "hex"))]
bitcoin_hashes::impl_debug_only_for_newtype!(TestType);

// this is the allocator the application will use
#[cfg(feature = "alloc")]
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[cfg(feature = "alloc")]
const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    #[cfg(feature = "alloc")]
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    let mut engine = sha256::Hash::engine();
    engine.write_all(b"abc").unwrap();
    #[cfg(feature = "hex")]
    check_result(engine);

    let mut engine = sha256::Hash::engine();
    engine.input(b"abc");
    #[cfg(feature = "hex")]
    check_result(engine);

    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

#[cfg(feature = "hex")]
fn check_result(engine: sha256::HashEngine) {
    let hash = TestType(sha256::Hash::from_engine(engine));

    let hash_check =
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad".parse::<TestType>()
            .unwrap();
    hprintln!("hash:{} hash_check:{}", hash, hash_check).unwrap();
    if hash != hash_check {
        debug::exit(debug::EXIT_FAILURE);
    }

    #[cfg(feature = "alloc")]
    if hash.to_string() != hash_check.to_string() {
        debug::exit(debug::EXIT_FAILURE);
    }
}
