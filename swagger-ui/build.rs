use std::path::PathBuf;
use anyhow::Context;
use async_recursion::async_recursion;
use serde::Deserialize;
use futures::StreamExt;
use reqwest::{Client, IntoUrl};

#[derive(Deserialize)]
#[serde(rename_all="snake_case")]
enum EntryType {
    File,
    Dir
}

#[derive(Deserialize)]
struct FolderEntry {
    name: String,
    url: String,
    r#type: EntryType,
    download_url: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    /// Single value
    One(T),
    /// Array of values
    Vec(Vec<T>),
}
impl<T> From<OneOrMany<T>> for Vec<T> {
    fn from(from: OneOrMany<T>) -> Self {
        match from {
            OneOrMany::One(val) => vec![val],
            OneOrMany::Vec(vec) => vec,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join(".dist");
    download_folder("https://api.github.com/repos/swagger-api/swagger-ui/contents/dist", out_dir).await?;
    Ok(())
}

#[async_recursion]
async fn download_folder(url: &str, to: impl Into<PathBuf> + Send + 'static) -> anyhow::Result<()> {
    let entries: Vec<_> = reqwest()?.get(url).send().await.with_context(||format!("failed to query folder data for {url}"))?.json::<OneOrMany<FolderEntry>>().await.with_context(||format!("failed to parse json for {url}"))?.into();
    let ref path = to.into();
    futures::future::try_join_all(entries.into_iter().map(|entry| async move {
        match entry.r#type {
            EntryType::File => download_file(entry.download_url.unwrap(), path.clone().join(entry.name)).await,
            EntryType::Dir => download_folder(&entry.url,  path.clone().join(entry.name)).await
        }
    })).await?;
    Ok(())
}

async fn download_file(url: impl IntoUrl + Send, to: impl Into<PathBuf> + Send) -> anyhow::Result<()> {
    let path = to.into();
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let mut byte_stream = reqwest()?.get(url).send().await?.bytes_stream();
    let mut tmp_file = tokio::fs::File::create(path).await?;
    while let Some(item) = byte_stream.next().await {
        tokio::io::copy(&mut item?.as_ref(), &mut tmp_file).await?;
    }
    Ok(())
}

fn reqwest()  -> anyhow::Result<Client> {
    Ok(Client::builder()
        .user_agent("reqwest")
        .build()?)
}