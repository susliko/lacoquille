use crate::Book;

pub async fn fetch_books(ids: &[u64]) -> anyhow::Result<Vec<Book>> {
    let client = reqwest::Client::new();
    let url = format!("https://gutendex.com/books?languages=fr");
    let resp: serde_json::Value = client.get(&url).send().await?.json().await?;

    let books: Vec<Book> = resp["results"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|b| {
            let id = b["id"].as_u64().unwrap_or(0);
            ids.contains(&id)
        })
        .map(|b| Book {
            id: b["id"].as_u64().unwrap_or(0),
            title: b["title"].as_str().unwrap_or("").to_string(),
            authors: b["authors"].as_array().map(|arr| {
                arr.iter().map(|a| a["name"].as_str().unwrap_or("").to_string()).collect()
            }).unwrap_or_default(),
            languages: b["languages"].as_array().map(|arr| {
                arr.iter().filter_map(|l| l.as_str().map(|s| s.to_string())).collect()
            }).unwrap_or_default(),
            download_count: b["download_count"].as_u64().unwrap_or(0),
        })
        .collect();

    Ok(books)
}
