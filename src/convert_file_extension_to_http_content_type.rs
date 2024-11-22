// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


/// # Summary
/// Converts file extension to appropriate HTTP content type.
///
/// # Arguments
/// - file_extension: File extension without leading dot.
///
/// # Returns
/// - HTTP content type assigned to file extension or None if file extension is not supported.
pub fn convert_file_extension_to_http_content_type(file_extension: &str) -> Option<String>
{
    return match file_extension
    {
        "gif" => Some("image/gif".to_owned()),
        "html" => Some("text/html".to_owned()),
        "jpeg" | "jpg" => Some("image/jpeg".to_owned()),
        "mp4" => Some("video/mp4".to_owned()),
        "png" => Some("image/png".to_owned()),
        "webm" => Some("video/webm".to_owned()),
        "webp" => Some("image/webp".to_owned()),
        _ => None
    }
}