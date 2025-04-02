// Copyright 2025 Kore Ledger 
// SPDX-License-Identifier: AGPL-3.0-or-later

// Extern functions for the wasm module.
unsafe extern "C" {
    // Host functions
    // Read the byte from the context indicated by the pointer
    pub(crate) fn read_byte(pointer: i32) -> u8;
    // Gets the length in bytes of the context structure starting with the indicated pointer
    pub(crate) fn pointer_len(pointer: i32) -> i32;
    // Reserve memory in the context state for later writes
    pub(crate) fn alloc(len: u32) -> i32;
    // Write a byte at the indicated position
    pub(crate) fn write_byte(ptr: u32, offset: u32, data: u8);
    // Println
    #[allow(dead_code)]
    pub(crate) fn cout(ptr: u32);
}
