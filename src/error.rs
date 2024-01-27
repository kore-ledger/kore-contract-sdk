// Copyright 2024 Antonio Estévez
// SPDX-License-Identifier: AGPL-3.0-or-later

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Serialization Error")]
    SerializationError,
    #[error("Deserialization Error")]
    DeserializationError,
}
