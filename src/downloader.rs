use crate::consts::constants;
use bytes::buf::Buf;
use reqwest::{
    blocking::Client,
    header::{ACCEPT, USER_AGENT},
};
use serde_json::Value;
use std::path::Path;

const FETCH_LATEST_MODULES_LINK: &str =
    "https://api.github.com/repos/Mee12345/GWater-V3/releases/latest";

pub fn download_files(garrysmod_path: &Path) {
    let lua_bin_path = garrysmod_path.join("garrysmod/lua/bin");
    let client = Client::new();
    let gwater_modules_json = get_gwater_modules_json(&client);
    let gwater_modules_file_name = get_gwater_modules_file_name(&gwater_modules_json);
    let current_directory = std::env::current_dir().expect("Could not get current directory");
    let temp_modules_file_location = current_directory.join(&gwater_modules_file_name);
    download_file(
        &client,
        &get_gwater_modules_download_link(&gwater_modules_json),
        &temp_modules_file_location,
        true,
    );
    let mut modules_zip = zip::ZipArchive::new(
        std::fs::File::open(&temp_modules_file_location).expect("Could not open zip modules file"),
    )
    .expect("Could not create zip file");
    modules_zip
        .extract(&current_directory)
        .expect("Could not extract files from a zip");
    for f in constants::LUA_BIN_MODULES {
        std::fs::rename(&current_directory.join(f), &lua_bin_path.join(f)).expect(&format!(
            "Could not move file {} to {}",
            &current_directory.join(f).display(),
            &lua_bin_path.join(f).display()
        ));
    }
    for f in constants::GARRYSMOD_MODULES {
        std::fs::rename(&current_directory.join(f), &garrysmod_path.join(f)).expect(&format!(
            "Could not move file {} to {}",
            &current_directory.join(f).display(),
            &garrysmod_path.join(f).display()
        ));
    }
    std::fs::remove_file(&temp_modules_file_location).expect("Could not remove temporary zip file");
}

fn get_gwater_modules_json(client: &Client) -> Value {
    let request = client
        .get(FETCH_LATEST_MODULES_LINK)
        .header(USER_AGENT, "gwater-installer/1.0")
        .send()
        .expect("Could not get gwater modules!");
    request.json().expect("Could not convert to json")
}

fn get_gwater_modules_download_link(request_json: &Value) -> &str {
    request_json["assets"][1]["browser_download_url"]
        .as_str()
        .expect("Could not convert json to string")
}

fn get_gwater_modules_file_name(request_json: &Value) -> &str {
    request_json["assets"][1]["name"]
        .as_str()
        .expect("Could not convert json to string")
}

fn download_file(client: &Client, file_link: &str, destination: &Path, stream: bool) {
    let mut request_bytes = request_file_bytes(client, file_link, stream)
        .expect("Could not request file")
        .bytes()
        .expect("Failed to convert to bytes")
        .reader();
    let mut destination_file = std::fs::File::create(destination).expect(&format!(
        "Could not create a destination file at {}",
        destination.display()
    ));
    std::io::copy(&mut request_bytes, &mut destination_file)
        .expect("Could not copy the url contents to created file");
}

fn request_file_bytes(
    client: &Client,
    file_link: &str,
    accept_stream: bool,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let mut request_builder = client
        .get(file_link)
        .header(USER_AGENT, "gwater-installer/1.0");
    request_builder = match accept_stream {
        true => request_builder.header(ACCEPT, "application/octet-stream"),
        false => request_builder,
    };
    let response = request_builder.send()?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::{get_gwater_modules_download_link, get_gwater_modules_json, request_file_bytes};

    #[test]
    fn test_request_file_bytes() {
        let client = reqwest::blocking::Client::new();
        let file_link =
            "https://github.com/Mee12345/GWater-V3/releases/download/1.4/GWater_Modules.zip";
        let accept_stream = true;
        request_file_bytes(&client, file_link, accept_stream).expect("Could not request bytes");
    }

    #[test]
    fn test_get_gwater_module_download_link() {
        let response = get_gwater_modules_json(&reqwest::blocking::Client::new());
        let link = get_gwater_modules_download_link(&response);
        assert_eq!(
            link,
            "https://github.com/Mee12345/GWater-V3/releases/download/1.4/GWater_Modules.zip"
        );
    }
}
