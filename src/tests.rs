#![cfg(test)]

use super::process::*;

#[test]
fn resolve_pointer_chain_empty() {
    let myself = Process::find(std::process::id()).unwrap();
    let result = myself.resolve_pointer_chain(&[]);
    assert!(result.is_err());
}

#[test]
fn resolve_pointer_chain_single() {
    let myself = Process::find(std::process::id()).unwrap();
    let result = myself.resolve_pointer_chain(&[myself.base_address]);
    assert_eq!(result.unwrap(), myself.base_address);
}

#[test]
fn resolve_pointer_chain_multiple() {
    let myself = Process::find(std::process::id()).unwrap();
    let target_value = 1337;
    let target_ptr = &target_value as *const i32 as usize;
    let base_ptr = &target_ptr as *const usize as usize;
    let address = myself.resolve_pointer_chain(&[base_ptr, 0x0, 0x0]).unwrap();
    assert_eq!(address, target_ptr);
    let value = myself.read_mem::<i32>(address).unwrap();
    assert_eq!(value, target_value);
}
