use chrono::NaiveDateTime;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
mod message;
use message::Message;
mod mailmessage;
use mailmessage::MailMessage;
mod error;
use error::Error;
mod attachment;
use futures_util::StreamExt;
use reqwest::Client;
use std::{fs::File, io::Write};

const API: &str = "https://www.1secmail.com/api/v1/";
const BANNED: [&str; 6] = [
    "abuse",
    "webmaster",
    "contact",
    "postmaster",
    "hostmaster",
    "admin",
];

fn from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}

#[derive(Debug)]
pub struct TempMail {
    email: String,
    mail_messages: Vec<MailMessage>,
    client: Client,
}

impl TempMail {
    pub fn new() -> Self {
        TempMail {
            email: "".to_string(),
            mail_messages: Vec::new(),
            client: Client::new(),
        }
    }

    pub async fn from_string<S>(email: S) -> Result<Self, Error>
    where
        S: Into<String> + Copy,
    {
        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

        let email_str = email.into();

        if re.is_match(&email_str) {
            let auth: Vec<&str> = email_str.split('@').collect();
            if !BANNED.contains(&auth[0])
                && Self::get_domains().await?.contains(&auth[1].to_string())
            {
                Ok(TempMail {
                    email: email_str,
                    mail_messages: Vec::new(),
                    client: Client::new(),
                })
            } else {
                Err(Error::new("Not valid email adress"))
            }
        } else {
            Err(Error::new("Not an email adress"))
        }
    }

    pub fn get_email(&self) -> String {
        return self.email.clone();
    }

    pub async fn generate_email(&mut self) -> Result<(), Error> {
        let response: Value = self
            .client
            .get(format!("{}?action=genRandomMailbox&count=1", API))
            .send()
            .await?
            .json()
            .await?;

        self.email = response[0]
            .as_str()
            .ok_or_else(|| Error::new("Failed to retrieve email from response"))?
            .into();

        Ok(())
    }

    pub async fn get_domains() -> Result<Vec<String>, Error> {
        let response = reqwest::get(format!("{}?action=getDomainList", API))
            .await?
            .text()
            .await?;
        let domains: Vec<String> = serde_json::from_str(&response)?;
        Ok(domains)
    }

    pub async fn get_adresses(count: Option<u32>) -> Result<Vec<String>, Error> {
        let count = match count {
            Some(val) => val,
            None => 1,
        };

        let response = reqwest::get(format!("{}?action=genRandomMailbox&count={}", API, count))
            .await?
            .text()
            .await?;

        let addresses: Vec<String> = serde_json::from_str(&response)?;

        Ok(addresses)
    }

    pub async fn check_inbox(&mut self) -> Result<(), Error> {
        let auth: Vec<&str> = self.email.split('@').collect();
        let response = reqwest::get(format!(
            "{}?action=getMessages&login={}&domain={}",
            API, auth[0], auth[1]
        ))
        .await?
        .text()
        .await?;

        self.mail_messages = serde_json::from_str(&response).unwrap();

        Ok(())
    }

    pub async fn get_message_by_id(&mut self, id: usize) -> Result<Message, Error> {
        let auth: Vec<&str> = self.email.split('@').collect();
        let response = self
            .client
            .get(format!(
                "{}?action=readMessage&login={}&domain={}&id={}",
                API, auth[0], auth[1], id
            ))
            .send()
            .await?;

        if let Ok(data) = response.json::<Message>().await {
            return Ok(data);
        } else {
            return Err(Error::new("Id not found"));
        };
    }

    pub async fn download_attachment(
        &mut self,
        id: usize,
        filename: String,
        path: String,
    ) -> Result<(), reqwest::Error> {
        let auth: Vec<&str> = self.email.split('@').collect();
        let mut response = self
            .client
            .get(format!(
                "{}?action=download&login={}&domain={}&id={}&file={}",
                API, auth[0], auth[1], id, filename
            ))
            .send()
            .await?
            .bytes_stream();

        let mut file = File::create(path).expect("ban");

        while let Some(item) = response.next().await {
            file.write_all(&item?);
        }

        Ok(())
    }

    pub fn get_messages_len(&self) -> usize {
        return self.mail_messages.len();
    }

    pub fn get_messages(&self) -> Vec<MailMessage> {
        return self.mail_messages.clone();
    }
}
