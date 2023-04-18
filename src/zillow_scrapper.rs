use reqwest::header::{self, HeaderMap};

use std::error::Error;

use crate::zillow_profile::ZillowEmbeddedJson;

pub async fn get_profile_data(name: &str) -> Result<ZillowEmbeddedJson, Box<dyn Error>> {
    let url = format!("https://www.zillow.com/profile/{}/", name);
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("authority", "www.zillow.com".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert("referer", "https://www.zillow.com/captchaPerimeterX/?url=%2fprofile%2fAdam%2520Merrick%2f&uuid=ff25e970-dd45-11ed-aba7-adfbba958639&vid=d03d7dbc-c8d5-11ed-96d2-426758766c78".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Chromium\";v=\"112\", \"Google Chrome\";v=\"112\", \"Not:A-Brand\";v=\"99\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert(header::COOKIE, "zgsession=1|3ead8780-1e94-4941-96c6-67396e6a994d; zguid=24|%2474a5ad65-ffe1-4c26-9766-61655fdd4882; AWSALB=acyYr1jXR1qqysHujPTss6eB35gjkWnVg0tKqUhRn9Ol1KW5QOadGSkRWAOk9VEU0yV+mOcMz8SylQjPA09qP1DpJT4Szd6wQMHHqQBYzQeG6wJ5sm8Vi53lpDa2; AWSALBCORS=acyYr1jXR1qqysHujPTss6eB35gjkWnVg0tKqUhRn9Ol1KW5QOadGSkRWAOk9VEU0yV+mOcMz8SylQjPA09qP1DpJT4Szd6wQMHHqQBYzQeG6wJ5sm8Vi53lpDa2; JSESSIONID=53AE52ADF98B032202C524C512EA8EF8; ZILLOW_SID=1|AAAAAVVbFRIBVVsVEjZaMfw0QOccKC1%2Bl1w4mr7UwBSQZY0cr4hUviYpEOPkkM60UBBPVNdvmyhz9JY%2Bjw9eKIfclMpa".parse().unwrap());

    let res = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let data = extract_data_from_html(&res)?;
    let parsed: ZillowEmbeddedJson = serde_json::from_str(&data)?;
    println!("{:#?}", parsed);
    Ok(parsed)
}
pub fn extract_data_from_html(html: &str) -> Result<String, Box<dyn Error>> {
    let data_regex =
        regex::Regex::new(r#"id="__NEXT_DATA__" type="application/json">(.+})</script"#)?;
    let data = data_regex
        .captures(html)
        .ok_or("Data not found")?
        .get(1)
        .ok_or("Capture group not found")?
        .as_str()
        .to_string();
    Ok(data)
}
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_works() {
        let name = "Adam Merrick";
        assert_eq!(super::get_profile_data(name).await.is_ok(), true);
    }
}
