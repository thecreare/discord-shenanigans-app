use poise::{CreateReply, Modal};
use serenity::all::{ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateAllowedMentions, CreateButton, CreateEmbed, CreateEmbedFooter};

use crate::{Context, Error};

#[derive(Debug, Modal)]
#[name = "Message Content"] // Struct name by default
struct ContentModal {
    #[name = "Content"] // Field name by default
	#[paragraph]
    #[max_length = 2000]
    content: String,
}

#[derive(Debug, Modal)]
#[name = "Edit Embed"] // Struct name by default
struct EmbedModal {
	#[max_length = 256]
	title: Option<String>,
    #[name = "Content"]
    #[paragraph] // Switches from single-line input to multiline text box
	#[max_length = 4000]
    content: Option<String>, // Option means optional input
	#[max_length = 2048]
	footer: Option<String>,
}

/// Send the changelog
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn embed_builder(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let components = CreateActionRow::Buttons(vec![
        CreateButton::new("edit.content")
            .label("Edit Content")
            .style(ButtonStyle::Primary)
            .emoji('📋'),
        CreateButton::new("edit.embed")
            .label("Edit Embed")
            .style(ButtonStyle::Secondary)
            .emoji('📋'),
    ]);

    let builder = CreateReply::default()
        .content("")
        .components(vec![components]);

    let reply = ctx.send(builder).await?;
	// ctx.guild_channel().await?.send_message(cache_http, builder)

	let mut content: String = "".to_string();

    while let Some(mci) = ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(60*10))
        .await
    {
		match mci.data.custom_id.as_str() {
			"edit.content" => {
				let defaults = Some(ContentModal { content: content.clone() });
				let data = poise::execute_modal_on_component_interaction::<ContentModal>(ctx, mci, defaults, None).await?;
				if let Some(data) = data {
					content = data.content.clone();
					reply.edit(ctx, CreateReply::default().content(data.content)).await?;
				}
			},
			"edit.embed" => {
				// let defaults = Some(EmbedModal { content: content.clone() });
				let defaults = None;
				let data = poise::execute_modal_on_component_interaction::<EmbedModal>(ctx, mci, defaults, None).await?;
				if let Some(data) = data {
					if data.title.is_none() && data.content.is_none() && data.footer.is_none() {
						reply.edit(ctx, CreateReply::default()).await?;
						continue;
					}
					let embed = CreateEmbed::new()
					.title(data.title.unwrap_or("".to_string()))
					.description(data.content.unwrap_or("".to_string()))
					.footer(CreateEmbedFooter::new(data.footer.unwrap_or("".to_string())));
					reply.edit(ctx, CreateReply::default().embed(embed)).await?;
				}
			}
			_ => {}
		}
    }

    // let image_embed = CreateEmbed::new()
    //     .image("attachment://editor.png")
    //     .description("Test it out here: <https://www.roblox.com/games/15514848643>");
    let embed1 = CreateEmbed::new().description("");
    let embed2 = CreateEmbed::new().description("");
    let builder = CreateReply::default()
        .content("")
        .embed(embed1)
        .embed(embed2)
        .allowed_mentions(CreateAllowedMentions::new().all_roles(true))
        // .ephemeral(true)
        // .attachment(CreateAttachment::path("./editor.png").await.unwrap())
        ;
    ctx.send(builder).await?;

    Ok(())
}
