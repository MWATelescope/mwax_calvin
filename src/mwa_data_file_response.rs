use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct MwaDataFileResponse {
    filetype: u16,
    size: u32,
    host: String,
    site_path: String,
    remote_archived: bool,
    deleted: bool,
    deleted_timestamp: Option<u64>,
    location: u16,
    prefix: String,
    bucket: String,
    folder: String,
    dl_name: String,
    dl_url: Option<String>,
}
