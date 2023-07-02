use crate::structures::{
    channels::message::{Reply, SendableEmbed},
    media::embeds::{Image, Metadata, Text, Video},
};

use super::embeds::Embed;

impl From<Embed> for SendableEmbed {
    fn from(value: Embed) -> Self {
        match value {
            Embed::Website(Metadata {
                url,
                title,
                description,
                icon_url,
                colour,
                ..
            }) => SendableEmbed {
                icon_url,
                url,
                title,
                description,
                colour,
                ..Default::default()
            },
            Embed::Image(Image { url, .. }) => SendableEmbed {
                url: Some(url),
                ..Default::default()
            },
            Embed::Video(Video { url, .. }) => SendableEmbed {
                url: Some(url),
                ..Default::default()
            },
            Embed::Text(Text {
                icon_url,
                url,
                title,
                description,
                colour,
                ..
            }) => SendableEmbed {
                icon_url,
                url,
                title,
                description,
                colour,
                ..Default::default()
            },
            Embed::None => SendableEmbed::default(),
        }
    }
}

pub struct ReplyWrapper(Option<Vec<Reply>>);

impl From<Option<Vec<String>>> for ReplyWrapper {
    fn from(value: Option<Vec<String>>) -> Self {
        ReplyWrapper(if let Some(replies) = value {
            let mut temp = Vec::new();

            replies
                .into_iter()
                .for_each(|id| temp.push(Reply { id, mention: false }));
            Some(temp)
        } else {
            None
        })
    }
}
