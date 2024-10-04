// Copyright 2024 Kore Ledger 
// SPDX-License-Identifier: AGPL-3.0-or-later

// Extern functions for the wasm module.
extern "C" {
    // Host functions
    // Lee el byte del contexto indicado por el puntero
    pub(crate) fn read_byte(pointer: i32) -> u8;
    // Obtiene la longitud en bytes de la estructura del contexto que empieza con el puntero indicado
    pub(crate) fn pointer_len(pointer: i32) -> i32;
    // Reserva memoria en el estado del contexto para posteriores escrituras
    pub(crate) fn alloc(len: u32) -> i32;
    // Escribe un byte en la posición indicada
    pub(crate) fn write_byte(ptr: u32, offset: u32, data: u8);
    // Println
    #[allow(dead_code)]
    pub(crate) fn cout(ptr: u32);
}
