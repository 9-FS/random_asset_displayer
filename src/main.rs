// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
mod config;
use config::*;
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
        Ok(_) => {return std::process::ExitCode::SUCCESS;} // no panic, program executed successfully
        Err(_) => {return std::process::ExitCode::FAILURE;} // panic, program failed
    };
}