// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use crate::config::*;
use crate::display_asset_random::*;
use crate::display_favicon::*;
use crate::error::*;


pub async fn main_inner(config: Config) -> Result<(), Error>
{
    const ASSETS_PATH: &str = "./assets/"; // path to assets
    let assets_filepath: Vec<String>; // list of assets filepath to load and display, contains all file
    let web_server; // web server


    assets_filepath = walkdir::WalkDir::new(ASSETS_PATH).follow_links(true).into_iter() // walk through assets directory
        .filter_map(|entry|
        {
            match entry
            {
                Ok(o) => // reading entry succeeded
                {
                    if o.file_type().is_dir() {return None;} // filter out directories

                    match o.path().to_str() // DirEntry -> String
                    {
                        Some(o) => Some(o.to_owned()), // converting succeeded, return filepath wrapped in Some to keep
                        None => // converting failed
                        {
                            log::warn!("Converting path \"{o:?}\" to &str failed.");
                            None
                        }
                    }
                }
                Err(e) => // reading entry failed
                {
                    log::warn!("{e}"); // log warning
                    None // return None to filter out
                }
            }
        })
        .filter(|entry| !entry.ends_with("/favicon.ico")) // filter out favicon.ico
        .collect();
    log::info!("Loaded asset list from \"{ASSETS_PATH}\".");
    log::debug!("{assets_filepath:?}");
    if assets_filepath.is_empty() {return Err(Error::AssetListEmpty());} // check if redirect list is empty


    match actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(assets_filepath.clone()))
            .service(display_asset_random) // "/" -> redirect
            .service(display_favicon) // "/favicon.ico" -> icon
    })
        .bind((config.HOST.clone(), config.PORT))
    {
        Ok(o) => web_server = o,
        Err(e) => return Err(Error::WebServerAddressBinding {host: config.HOST, port: config.PORT, reason: e}),
    }
    log::info!("Bound web server to \"{}:{}\".", config.HOST, config.PORT);

    web_server.run().await.expect("Running web server failed even though web server had already been bound successfully.");

    return Ok(());
}