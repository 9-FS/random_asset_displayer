// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


/// # Summary
/// Collection of settings making up the configuration of the application.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[allow(non_snake_case)]
pub struct Config
{
    pub DEBUG: Option<bool>, // debug mode?
    pub HOST: String, // web server host
    pub PORT: u16, // web server port
}

impl Default for Config
{
    fn default() -> Self
    {
        Config
        {
            DEBUG: None, // no entry in default config, defaults to false
            HOST: "::".to_owned(),
            PORT: 80
        }
    }
}