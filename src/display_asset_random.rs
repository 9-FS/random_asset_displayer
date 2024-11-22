// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use rand::seq::IteratorRandom;
use crate::convert_file_extension_to_http_content_type::*;


/// # Summary
/// Picks a random filepath, loads the filecontent, and displays it. Display mode depends on file type. Currently supported are:
/// - raw HTML
/// - images: *.gif, *.jpg, *.jpeg, *.png, *.webp
/// - videos: *.mp4, *.webm
///
/// # Returns
/// HTTP response with file content
#[actix_web::get("/")]
pub async fn display_asset_random(assets_filepath: actix_web::web::Data<Vec<String>>) -> impl actix_web::Responder
{
    let asset_random_filepath: String; // random line from asset filepath list
    let r; // web server response


    asset_random_filepath = assets_filepath
        .to_vec()
        .iter()
        .choose(&mut rand::thread_rng()) // pick random line from redirect list or if empty default to status code 500
        .expect("Asset list is empty even though it was checked before to not be empty.")
        .to_owned();

    r = match std::fs::read(&asset_random_filepath) // load asset filecontent
    {
        Ok(o) => // loading file succeeded
        {
            match convert_file_extension_to_http_content_type(asset_random_filepath.split('.').last().unwrap_or("html")) // parse file extension and convert to HTTP content type
            {
                Some(s) => // supported filetype
                {
                    actix_web::HttpResponse::Ok()
                        .content_type(s.clone())
                        .body(o)

                },
                None => // unsupported filetype
                {
                    log::error!("Filetype of \"{asset_random_filepath}\" is not supported.");
                    actix_web::HttpResponse::NoContent() // return error 500 response
                        .status(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                        .content_type(actix_web::http::header::ContentType::html())
                        .body(format!("<center><h1>Filetype of \"{asset_random_filepath}\" is not supported.</h1></center>"))
                }
            }
        }
        Err(e) => // loading file failed
        {
            log::error!("Loading file from \"{asset_random_filepath}\" failed with: {e}.");
            actix_web::HttpResponse::NoContent() // return error 500 response
                .status(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                .content_type(actix_web::http::header::ContentType::html())
                .body(format!("<center><h1>Loading file from \"{asset_random_filepath}\" failed with: {e}.</h1></center>"))
        },
    };

    return r;
}