use reqwest::header::{self, HeaderMap};
use std::collections::HashMap;
use tracing::info;

macro_rules! memoize {
    ($func:expr) => {{
        let mut cache = HashMap::new();
        move |arg| {
            if let Some(result) = cache.get(arg) {
                return result.clone();
            }
            let result = $func(arg);
            cache.insert(arg.clone(), result.clone());
            result
        }
    }};
}

use std::error::Error;

use crate::zillow_profile::ZillowEmbeddedJson;

pub async fn get_profile_data(name: &str) -> Result<ZillowEmbeddedJson, Box<dyn Error>> {
    let url = format!("https://www.zillow.com/profile/{}/", name);
    println!("url: {}", &url);
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
    let json_raw = extract_data_from_html(&res)?;
    let parsed: ZillowEmbeddedJson = serde_json::from_str(&json_raw)?;
    Ok(parsed)
}
pub fn extract_data_from_html(html: &str) -> Result<String, Box<dyn Error>> {
    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    Ok(dom
        .get_element_by_id("__NEXT_DATA__")
        .ok_or_else(|| "No element with id '__NEXT_DATA__' found")?
        .get(parser)
        .ok_or_else(|| "Element with id '__NEXT_DATA__' has no inner text")?
        .inner_text(parser)
        .into())
}
pub async fn get_profile_data_with_redis_cache(
    name: &str,
) -> Result<ZillowEmbeddedJson, Box<dyn Error>> {
    // Check if the profile data is already cached in Redis
    if let Some(cached_data) = get_from_redis(name).await? {
        return Ok(cached_data);
    }
    let profile_data = get_profile_data(name).await?;

    // Cache the profile data in Redis
    cache_in_redis(name, &profile_data).await?;

    Ok(profile_data)
}

async fn get_from_redis(name: &str) -> Result<Option<ZillowEmbeddedJson>, Box<dyn Error>> {
    // Connect to Redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_async_connection().await?;

    // Get the cached data from Redis
    let cached_data: Option<String> = redis::cmd("GET").arg(name).query_async(&mut conn).await?;
    if let Some(data) = cached_data {
        let parsed: ZillowEmbeddedJson = serde_json::from_str(&data)?;
        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}

async fn cache_in_redis(name: &str, data: &ZillowEmbeddedJson) -> Result<(), Box<dyn Error>> {
    // Connect to Redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_async_connection().await?;

    // Cache the data in Redis
    let json_data = serde_json::to_string(data)?;
    redis::cmd("SET")
        .arg(name)
        .arg(json_data)
        .query_async::<_, ()>(&mut conn)
        .await?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::zillow_scrapper::{get_profile_data, get_profile_data_with_redis_cache};

    #[tokio::test]
    async fn it_works() {
        let name = "Matt Laricy";
        let result = get_profile_data_with_redis_cache(name).await;
        println!("result: {:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}
