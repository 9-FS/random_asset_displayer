// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
mod config;
use config::*;
mod error;
mod main_inner;
use main_inner::*;


fn main() -> std::process::ExitCode
{
    const DEBUG: bool = false; // debug mode?
    let config: Config; // config, settings
    let tokio_rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().expect("Creating tokio runtime failed."); // async runtime


    if DEBUG == true // setup logging
    {
        setup_logging::setup_logging(log::Level::Debug, None, "./log/%Y-%m-%dT%H_%M.log");
    }
    else
    {
        setup_logging::setup_logging(log::Level::Info, None, "./log/%Y-%m-%d.log");
    }

    std::panic::set_hook(Box::new(|panic_info: &std::panic::PanicInfo| // override panic behaviour
    {
        log::error!("{}", panic_info); // log panic source and reason
        log::error!("{}", std::backtrace::Backtrace::capture()); // log backtrace
    }));

    match load_config::load_config
    (
        vec!
        [
            load_config::Source::Env,
            load_config::Source::File(load_config::SourceFile::Toml("./config/.env".to_string())),
        ],
        Some(load_config::SourceFile::Toml("./config/.env".to_string()))
    )
    {
        Ok(o) => {config = o;} // loaded config successfully
        Err(_) => {return std::process::ExitCode::FAILURE;} // loading config failed
    }


    match std::panic::catch_unwind(|| tokio_rt.block_on(main_inner(config))) // execute main_inner, catch panic
    {
        Ok(result) => // no panic
        {
            match result
            {
                Ok(()) => {return std::process::ExitCode::SUCCESS;} // program executed successfully
                Err(e) => // program failed in a controlled manner
                {
                    match e // log error
                    {
                        error::Error::RedirectListEmpty() => log::error!("{e}"),
                        error::Error::StdIo(e) => log::error!("Loading redirect list failed with: {e}"),
                        error::Error::WebServerAddressBinding {host, port, reason} => log::error!("Binding web server to \"{host}:{port}\" failed with: {reason}"),
                    }
                    return std::process::ExitCode::FAILURE;
                }
            }
        }
        Err(_) => {return std::process::ExitCode::FAILURE;} // program crashed with panic, dis not good
    };
}