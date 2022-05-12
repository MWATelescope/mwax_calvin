use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MwaDataFileResponse {
    filetype: u16,
    size: u32,
    host: String,
    site_path: String,
    remote_archived: bool,
    deleted: bool,
    deleted_timestamp: u64,
    location: u16,
    prefix: String,
    bucket: String,
    folder: String,
    dl_name: String,
    dl_url: String,
}

#[derive(Deserialize, Debug)]
struct MwaDataFilesResponse {
    filename: String,
    details: MwaDataFileResponse,
}

fn main() -> Result<(), Error> {
    let client = Client::new();

    let request_url = format!(
        "http://ws.mwatelescope.org/metadata/data_files?obs_id={obs_id}",
        obs_id = 1096952256
    );

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    let response_result = client.get(request_url).headers(headers).send();

    if let Ok(r) = response_result {
        println!("Response returned. {:?}", r);
        println!("Checking status code: {}", r.status());

        if r.status().is_success() {
            println!("All is good!");
            let files: Vec<MwaDataFilesResponse> = r.json()?;

            println!("{:?}", files);
        } else {
            println!("The error is: \n{:?}", r.error_for_status());
        }
    } else {
        println!("Error {:?}", response_result.unwrap_err());
    }

    Ok(())
}
