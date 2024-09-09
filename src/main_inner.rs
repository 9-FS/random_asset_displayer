// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use crate::config::*;
use crate::error::*;
use rand::seq::IteratorRandom;


pub async fn main_inner(config: Config) -> Result<()>
{
    const REDIRECT_LIST_FILEPATH: &str = "./config/redirect_list.txt";
    let redirect_list: actix_web::web::Data<Vec<String>>; // list of url to redirect to or http status code to simply return
    let web_server; // web server


    redirect_list = actix_web::web::Data::new(std::fs::read_to_string(REDIRECT_LIST_FILEPATH)?.lines().filter(|line| !line.is_empty()).map(|line| line.to_owned()).collect()); // load redirect list, remove empty lines, &str -> String
    log::info!("Loaded redirect list from \"{REDIRECT_LIST_FILEPATH}\".");
    log::debug!("{redirect_list:?}");
    if redirect_list.to_vec().is_empty() {return Err(Error::RedirectListEmpty());} // check if redirect list is empty


    match actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(redirect_list.clone())
            .route("/", actix_web::web::get().to(redirect)) // "/" -> redirect
            .route("/favicon.ico", actix_web::web::get().to(favicon)) // "/favicon.ico" -> icon
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


/// # Summary
/// Picks a random line from the redirect list and returns an empty HTTP response with the status code from the line or a redirect to the URL from the line.
///
/// # Returns
/// HTTP response
async fn redirect(redirect_list: actix_web::web::Data<Vec<String>>) -> impl actix_web::Responder
{
    let line_random: String; // random line from redirect list
    let response: actix_web::HttpResponse; // http resonse to answer


    line_random = redirect_list
        .to_vec()
        .iter()
        .choose(&mut rand::thread_rng()) // pick random line from redirect list or if empty default to status code 500
        .expect("Redirect list is empty even though it was checked before to not be empty.")
        .to_owned();


    match line_random.parse::<u16>() // try line -> u16
    {
        Ok(o) => // display status code
        {
            let status: actix_web::http::StatusCode = actix_web::http::StatusCode::from_u16(o) // try u16 -> http status code
            .unwrap_or_else(|_|
            {
                log::error!("Assigning a HTTP status code to {o} failed. Falling back to status 500 \"Internval Server Error\".");
                return actix_web::http::StatusCode::INTERNAL_SERVER_ERROR; // default to 500
            });

            response = actix_web::HttpResponse::NoContent()
                .status(status)
                .content_type(actix_web::http::header::ContentType::html())
                .body(format!("<center><h1>{status}</h1></center>" ));
        },
        Err(_) => { response = actix_web::HttpResponse::Found().append_header(("Location", line_random)).finish(); } // redirect to URL
    };

    return response;
}


/// # Summary
/// Loads the favicon from assets and returns it for browser tab icon.
///
/// # Returns
/// HTTP response with favicon file
async fn favicon() -> impl actix_web::Responder
{
    const FAVICON_FILEPATH: &str = "./assets/favicon.ico";


    return actix_files::NamedFile::open_async(FAVICON_FILEPATH).await;
}