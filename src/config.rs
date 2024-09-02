// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


/// # Summary
/// Collection of settings making up the configuration of the application.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(non_snake_case)]
pub struct Config
{
    pub HOST: String, // web server host
    pub PORT: u16, // web server port
}

impl Default for Config
{
    fn default() -> Self
    {
        Config
        {
            HOST: "::".to_owned(),
            PORT: 6900
        }
    }
}