use reqwest::blocking::Client;
use reqwest::Error;
mod mwa_data_file_response;
mod ws;

fn main() -> Result<(), Error> {
    let obs_id = 1096952256;

    let client = Client::new();

    let ws_result = ws::get_observation_data_files(&client, obs_id)?;

    let files = ws_result.unwrap();

    for f in &files {
        println!("{}", f.0);
    }

    Ok(())
}
