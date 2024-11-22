// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


#[derive(Debug, thiserror::Error)]
pub enum Error
{
    #[error("Asset list is empty.")]
    AssetListEmpty(),

    #[error("Binding web server to \"{host}:{port}\" failed with: {reason}")]
    WebServerAddressBinding {host: String, port: u16, reason: std::io::Error},
}