// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


/// # Summary
/// Loads the favicon from assets and returns it for browser tab icon.
///
/// # Returns
/// HTTP response with favicon file
#[actix_web::get("/favicon.ico")]
pub async fn display_favicon() -> impl actix_web::Responder
{
    const FAVICON_FILEPATH: &str = "./assets/favicon.ico";


    return actix_files::NamedFile::open_async(FAVICON_FILEPATH).await;
}