use crate::mwa_data_file_response::MwaDataFileResponse;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Error;
use std::collections::HashMap;

fn call_webservice(
    client: &reqwest::blocking::Client,
    request_url: &str,
) -> Result<Response, Error> {
    let mut headers = HeaderMap::new();

    // Add a HTTP_USER_AGENT- some web services require this
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    client.get(request_url).headers(headers).send()
}

pub(crate) fn get_observation_data_files(
    client: &Client,
    obs_id: u64,
) -> Result<Option<HashMap<String, MwaDataFileResponse>>, Error> {
    let url = format!(
        "http://ws.mwatelescope.org/metadata/data_files?obs_id={obs_id}",
        obs_id = obs_id
    );

    let response_result = call_webservice(client, &url);

    if let Ok(r) = response_result {
        println!("Response returned. {:?}", r);
        println!("Checking status code: {}", r.status());

        if r.status().is_success() {
            println!("All is good!");
            let files: HashMap<String, MwaDataFileResponse> = r.json()?;
            return Ok(Some(files));
        } else {
            return Err(r.error_for_status().unwrap_err());
        }
    } else {
        return Err(response_result.unwrap_err());
    }
}
