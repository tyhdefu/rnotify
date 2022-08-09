use std::error::Error;
use std::fmt::Display;
use chrono::{SecondsFormat, TimeZone};
use curl::easy::{Easy, List};
use serde::{Serialize, Deserialize};
use crate::destinations::MessageDestination;
use crate::error::MessageSendError;
use crate::Level;
use crate::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordDestination {
    url: String,
    username: Option<String>,
    #[serde(default = "default_message_content")]
    message_content: String,
}

fn default_message_content() -> String {
    return String::from("@RNotify");
}

impl MessageDestination for DiscordDestination {
    fn send<TZ: TimeZone>(&self, message: &Message<TZ>) -> Result<(), Box<dyn Error>>
        where TZ::Offset: Display {
        /*let mut dsc_message = discord_message::DiscordMessage::default();
        dsc_message.content = self.message_content.clone();

        let mut embed = discord_message::Embed::default();
        embed.footer = Some(EmbedFooter::default());
        embed.color = Some(get_color_from_level(message.get_level()));
        embed.title = message.get_title().as_deref().unwrap_or("").to_owned();
        if let Some(author) = message.get_author() {
            let mut dsc_author = EmbedAuthor::default();
            dsc_author.name = author.clone();
            embed.author = Some(dsc_author);
        }
        dsc_message.embeds = vec![embed];
        discord_message::send_message();*/
        let mut discord_msg = discord_webhook::models::Message::new();
        discord_msg.content(&self.message_content);
        discord_msg.embed(|embed| {
            embed.timestamp(&message.get_timestamp().to_rfc3339_opts(SecondsFormat::Millis, true));
            embed.description(message.get_message_detail());
            if let Some(author) = message.get_author() {
                embed.author(author, None, None);
            }
            embed.title(message.get_title().as_deref().unwrap_or("Rnotify Notification"));
            let color = get_color_from_level(message.get_level());
            embed.color(&format!("{}", color));
            return embed;
        });
        let payload = serde_json::to_string(&discord_msg)?;
        //println!("{}", payload);
        let payload_bytes = payload.as_bytes();

        let mut easy = Easy::new();
        easy.url(&self.url)?;

        let mut headers = List::new();
        headers.append("Accept: application/json")?;
        headers.append("Content-Type:application/json")?;
        easy.http_headers(headers)?;

        easy.post(true)?;
        easy.post_field_size(payload_bytes.len() as u64)?;
        easy.post_fields_copy(payload_bytes)?;
        //easy.perform()?;
        let mut response_buf = Vec::new();
        {
            let mut transfer = easy.transfer();
            /*transfer.read_function(move |into| {
                let num = payload_bytes.read(into).unwrap();
                println!("Read {}", num);
                Ok(num)
            })?;*/
            transfer.write_function(|buf| {
                response_buf.extend_from_slice(buf);
                Ok(buf.len())
            })?;
            transfer.perform()?;
        }
        let code = easy.response_code()?;
        if code != 200 && code != 204 {
            let response = String::from_utf8(response_buf)?;
            return Err(Box::new(MessageSendError::new(format!("Got response code {}: Response body: {}", code, response))));
        }
        Ok(())
    }
}

fn get_color_from_level(level: &Level) -> u32 {
    match level {
        Level::Info => 0x00F4D0,
        Level::Warn => 0xFFFF00,
        Level::Error => 0xFF0000,
        Level::SelfError => 0xB30000,
    }
}