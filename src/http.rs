use reqwest::Client;

use crate::Methods;

pub async fn run_http_method(url: String, current_method: Methods, body: String) -> String {
    match current_method {
        Methods::Get => get(&url).await,
        Methods::Post => post(&url, body).await,
        Methods::Put => put(&url, body).await,
        Methods::Delete => delete(&url, body).await,
        Methods::Patch => patch(&url, body).await,
        Methods::Head => head(&url).await,
    }
}

async fn get(url: &str) -> String {
    match reqwest::get(url).await {
        Ok(r) => match r.text().await {
            Ok(t) => format!("{}", t),
            Err(e) => format!("{}", e),
        },
        Err(e) => format!("{}", e),
    }
}

async fn post(url: &str, body: String) -> String {
    let client = Client::new();

    match client.post(url).body(body).send().await {
        Ok(r) => match r.text().await {
            Ok(t) => format!("{}", t),
            Err(e) => format!("{}", e),
        },
        Err(e) => format!("{}", e),
    }
}

async fn put(url: &str, body: String) -> String {
    let client = Client::new();

    match client.put(url).body(body).send().await {
        Ok(r) => match r.text().await {
            Ok(t) => format!("{}", t),
            Err(e) => format!("{}", e),
        },
        Err(e) => format!("{}", e),
    }
}

async fn delete(url: &str, body: String) -> String {
    let client = Client::new();

    match client.delete(url).body(body).send().await {
        Ok(r) => match r.text().await {
            Ok(t) => format!("{}", t),
            Err(e) => format!("{}", e),
        },
        Err(e) => format!("{}", e),
    }
}

async fn patch(url: &str, body: String) -> String {
    let client = Client::new();

    match client
        .patch(url)
        .body(body)
        .header("key", "value")
        .send()
        .await
    {
        Ok(r) => match r.text().await {
            Ok(t) => format!("{}", t),
            Err(e) => format!("{}", e),
        },
        Err(e) => format!("{}", e),
    }
}

async fn head(url: &str) -> String {
    let client = Client::new();

    match client.head(url).send().await {
        Ok(r) => match r.text().await {
            Ok(t) => format!("{}", t),
            Err(e) => format!("{}", e),
        },
        Err(e) => format!("{}", e),
    }
}
