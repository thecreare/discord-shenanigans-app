use poise::CreateReply;
use serenity::all::{Attachment, CreateEmbed};

use crate::{Context, Error};

const EMBED_CONTENT_LIMIT: usize = 4096;

fn try_push(container: &mut Vec<String>, value: String) {
    if let Some(thing) = container.last() && thing.len() + value.len() <= EMBED_CONTENT_LIMIT {
        let last = container.pop().unwrap();
        container.push(format!("{}{}", last, value));
    } else {
        container.push(value);
    }
}

#[poise::command(prefix_command, slash_command)]
pub async fn split_md_file(
    ctx: Context<'_>,
    #[description = "File to send as multiple embeds"] file: Attachment,
) -> Result<(), Error> {
    if !file.content_type.clone().ok_or("No content type")?.starts_with("text") {
        return Err(Error::from(format!("Invalid content type. Expected `text*` got `{}`", file.content_type.unwrap())));
    };

    println!("Downloading...");
    let bytes = file.download().await?;
    let string = String::from_utf8_lossy(&bytes);
    println!("Iterating...");

    let mut embed_contents: Vec<String> = vec![];
    let mut this_section = "\n".to_string();

    for line in string.lines() {
        if line.starts_with("#") && this_section.len() > 1 {
            try_push(&mut embed_contents, this_section);
            this_section = "\n".to_string();
        }
        this_section = format!("{}{}\n", this_section, line);
    }
    try_push(&mut embed_contents, this_section);

    if embed_contents.len() > 10 {
        return Err(Error::from("Too big"));
    }

    let mut reply = CreateReply::default();
    let mut embeds: Vec<CreateEmbed> = vec![];

    println!("Embedding....");
    for content in embed_contents {
        assert!(content.len() <= EMBED_CONTENT_LIMIT, "Embed too long"); // Can't happen
        let embed = CreateEmbed::new().description(content);
        embeds.push(embed);
    }

    reply.embeds = embeds;

    println!("Sending...");
    ctx.send(reply).await?;

    Ok(())
}
