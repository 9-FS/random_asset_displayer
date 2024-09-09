// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


#[derive(Debug, thiserror::Error)]
pub enum Error
{

    #[error("Redirect list is empty.")]
    RedirectListEmpty(),

    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error("Binding web server to \"{host}:{port}\" failed with: {reason}")]
    WebServerAddressBinding {host: String, port: u16, reason: std::io::Error},
}


pub type Result<T> = std::result::Result<T, Error>; // strict error handling, only takes pre defined Error type