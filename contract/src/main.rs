#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::Parameters, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Key, RuntimeArgs, URef,
};

#[no_mangle]
pub extern "C" fn hello_world() {
    // Doesn't advance RNG of the runtime
    // runtime::ret(CLValue::from_t("Hello, world!").unwrap_or_revert())
    runtime::put_key("hello", storage::new_uref("world").into());
}

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = {
        let mut entry_points = EntryPoints::new();

        let entry_point = EntryPoint::new(
            "hello_world",
            Parameters::default(),
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );

        entry_points.add_entry_point(entry_point);

        entry_points
    };
    let (contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        None,
        Some("contract_package_hash".into()),
        None,
    );

    runtime::put_key("contract_hash", contract_hash.into());
}
