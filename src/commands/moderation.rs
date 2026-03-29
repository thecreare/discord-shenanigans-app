use poise::CreateReply;
use serenity::all::{ChannelId, CreateAllowedMentions, CreateEmbed, CreateMessage, Message, MessageReference, MessageReferenceKind};

use crate::{Context, Error};

const REPORTS_CHANNEL: ChannelId = ChannelId::new(1487663981341573240);

#[poise::command(
    context_menu_command = "Report",
    // install_context = "Guild",
    // interaction_context = "Guild"
)]
pub async fn report_context(ctx: Context<'_>, msg: Message) -> Result<(), Error> {
    if msg.author.id == ctx.author().id {
        ctx.reply("Reporting yourself seems like a bad idea...").await?;
        return Ok(());
    }

    if msg.author.id == ctx.framework().bot_id {
        ctx.reply("Why would you want to report our glorious bot overlord?").await?;
        return Ok(());
    }
    

    msg.channel_id.send_message(ctx, CreateMessage::new()
        .content("This message has been reported, a moderator will take a look at it when they get a chance.".to_owned()
            + "\n-# Deleting your message won't change the verdict"
            + "\n-# To report a message, right click or long press the message you would like to report, select the Apps button, find Enactor of Shenanigans, and press the Report button"
        )
        .reference_message(&msg)
        .allowed_mentions(CreateAllowedMentions::new())
    ).await?;
    ctx.send(CreateReply::default().ephemeral(true).content("Thank you for your report.")).await?;
    

    REPORTS_CHANNEL.send_message(ctx, CreateMessage::new()
        .content(format!("_ _\nMessage author: <@{}>\nReport submitted by <@{}>", msg.author.id, ctx.author().id))
        .allowed_mentions(CreateAllowedMentions::new())
    ).await?;
    REPORTS_CHANNEL.send_message(ctx, CreateMessage::new()
        .reference_message(MessageReference::new(MessageReferenceKind::Forward, msg.channel_id).message_id(msg.id))
    ).await?;
    Ok(())
}
