use anyhow::{bail, Context, Result};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, Response,
};
use serde::Deserialize;
use std::time::SystemTime;

pub type Files = (String, File);
pub type FilesVec = Vec<Files>;

#[derive(Deserialize, Debug)]
pub struct File {
    pub filename: String,
    pub raw_url: String,
    pub size: u32,
    pub truncated: bool,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubResponse {
    pub description: String,
    #[serde(with = "tuple_vec_map")]
    pub files: Vec<(String, File)>,
}

fn parse_header_value(headers: &HeaderMap, name: &str, default: i64) -> i64 {
    let header = match headers.get(name) {
        Some(header) => header,
        None => return default,
    };

    header
        .to_str()
        .unwrap_or("")
        .parse::<i64>()
        .unwrap_or(default)
}

fn time_difference(older: u64) -> String {
    let start = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if older < start {
        return "undefined time".to_string();
    }
    let diff = older - start;
    format!("{}m {}s", diff / 60, diff % 60)
}

fn setup_request_client(token: Option<&str>) -> Result<Client> {
    let mut headers = HeaderMap::new();

    headers.insert(header::USER_AGENT, HeaderValue::from_static("rust-client"));
    headers.insert(
        header::ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    // add token as header
    let token = token.unwrap_or("");
    if !token.is_empty() {
        let token_str = format!("token {}", token);
        let header_value =
            HeaderValue::from_str(&token_str).with_context(|| "failed to create header value")?;
        headers.insert(header::AUTHORIZATION, header_value);
    }

    Client::builder()
        .default_headers(headers)
        .build()
        .with_context(|| "failed to create http client")
}

async fn parse_github_response(response: Response) -> Result<FilesVec> {
    match response.status().as_u16() {
        200 => match response.json::<GithubResponse>().await {
            Ok(res) => {
                let mut files = res.files;
                files.sort_by_key(|a| a.1.truncated);
                Ok(files)
            }
            Err(err) => {
                bail!("some error occurred while parsing github response. error: {err}")
            }
        },
        403 => {
            let headers = response.headers();
            let request_limit = parse_header_value(headers, "x-ratelimit-remaining", -1);

            if request_limit <= 0 {
                let time_until_reset = parse_header_value(headers, "x-ratelimit-reset", 0);
                let msg = format!("uh oh, looks like your github api quota has expired. use a token or wait for {}", time_difference(time_until_reset as u64));
                bail!(msg)
            } else {
                bail!("github sent 403 :( try using a token".to_string())
            }
        }
        404 => bail!("uh oh, looks like gist id not found".to_string()),
        _ => {
            bail!("unknown error occurred while parsing response, please open an issue".to_string())
        }
    }
}

pub async fn fetch_gist(gist_id: &str, token: Option<&str>) -> Result<FilesVec> {
    let client = setup_request_client(token)?;
    let url = format!("https://api.github.com/gists/{gist_id}");
    let response = client.get(&url).send().await?;
    Ok(parse_github_response(response).await?)
}

pub async fn download_file(url: &str, token: Option<&str>) -> Result<String> {
    let client = setup_request_client(token)?;
    let response = client.get(url).send().await?;
    Ok(response.text().await?)
}
