use rss::Channel;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum FetchError {
    Reqwest(reqwest::Error),
    Rss(rss::Error),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchError::Reqwest(err) => write!(f, "{}", err),
            FetchError::Rss(err) => write!(f, "{}", err),
        }
    }
}

impl StdError for FetchError {}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> FetchError {
        FetchError::Reqwest(err)
    }
}

impl From<rss::Error> for FetchError {
    fn from(err: rss::Error) -> FetchError {
        FetchError::Rss(err)
    }
}

pub async fn fetch_rss(url: &str) -> Result<Channel, FetchError> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
