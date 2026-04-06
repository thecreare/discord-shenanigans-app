use std::collections::HashSet;

use poise::CreateReply;
use serenity::all::{ButtonStyle, ChannelId, ComponentInteractionCollector, CreateActionRow, CreateAllowedMentions, CreateButton, CreateInteractionResponseMessage, CreateMessage, EditMessage, Mentionable, Message, MessageReference, MessageReferenceKind, UserId};

use crate::{Context, Error};

const REPORTS_CHANNEL: ChannelId = ChannelId::new(1487663981341573240);

fn get_report_header(ctx: &Context<'_>, msg: &Message, corroborators: Option<&HashSet<UserId>>) -> String {
    if let Some(corroborators) = corroborators {
        let c: String = corroborators.into_iter().map(|userid| userid.mention().to_string() + " ").collect();
        return format!("## User-submitted Report\nSubmitted by {}\nCorroborators: {}\nAuthor: {}", ctx.author().mention(), c, msg.author.mention())
    }
    format!("## User-submitted Report\nSubmitted by {}\nAuthor: {}", ctx.author().mention(), msg.author.mention())
}

#[poise::command(
    context_menu_command = "Report",
    install_context = "Guild",
    interaction_context = "Guild"
)]
pub async fn report_context(ctx: Context<'_>, msg: Message) -> Result<(), Error> {
    if msg.author.id == ctx.author().id {
        ctx.send(CreateReply::default().ephemeral(true).content("Reporting yourself seems like a bad idea...")).await?;
        return Ok(());
    }

    if msg.author.id == ctx.framework().bot_id {
        ctx.send(CreateReply::default().ephemeral(true).content("I can't believe you would try to report me :(")).await?;
        return Ok(());
    }

    let mut report_header = REPORTS_CHANNEL.send_message(ctx, CreateMessage::new()
        .content(get_report_header(&ctx, &msg, None))
        .allowed_mentions(CreateAllowedMentions::new())
    ).await?;

    REPORTS_CHANNEL.send_message(ctx, CreateMessage::new()
        .reference_message(MessageReference::new(MessageReferenceKind::Forward, msg.channel_id).message_id(msg.id))
        .allowed_mentions(CreateAllowedMentions::new())
    ).await?;

    let components = CreateActionRow::Buttons(vec![
        CreateButton::new("agree")
            .label("I agree with this report")
            .style(ButtonStyle::Secondary)
            .emoji('✅'),
        CreateButton::new("learn")
            .label("Learn more")
            .style(ButtonStyle::Secondary)
            .emoji('❓'),
    ]);

    let mut public_report_message = msg.channel_id.send_message(ctx, CreateMessage::new()
        .content("This message has been reported by a user in chat")
        .components(vec![components])
        .reference_message(&msg)
        .allowed_mentions(CreateAllowedMentions::new())
    ).await?;
    ctx.send(CreateReply::default().ephemeral(true).content("Thank you for your report")).await?;

    let mut corroborators: HashSet<UserId> = HashSet::new();
    while let Some(mci) = ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(60*10))
        .await
    {
		match mci.data.custom_id.as_str() {
            "learn" => {
                let mention = ctx.framework().bot_id.mention();
                mci.create_response(ctx, serenity::all::CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                    .content(format!("This is a bot created to let everyone help contribute to moderation.
Don't worry if your message was falsely reported, \
reports get sent sent to moderators for review and nothing happens unless the message actually breaks a rule.
Deleting a reported message has no effect.
Moderators read the context of messages around the report.
Clicking the \"I agree\" button indicates you agree the report is valid.

**Reporting a message:**
**1.** (Desktop) Right click the message
**1.** (Mobile) Long press the message
**2.** Select `Apps`
**3.** Find {mention} and press Report"))
                    .ephemeral(true)
                )).await?;
			},
			"agree" => {
                if mci.user == *ctx.author() {
                    mci.create_response(ctx, serenity::all::CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                        .content("You created this report, it is already assumed you agree with it")
                        .ephemeral(true)
                    )).await?;
                    continue
                }
                let id = mci.user.id;
                if corroborators.contains(&id) {
                    mci.create_response(ctx, serenity::all::CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                        .content("You have already agreed with this report")
                        .ephemeral(true)
                    )).await?;
                    continue
                }
                corroborators.insert(id);
                report_header.edit(ctx, EditMessage::new()
                    .content(get_report_header(&ctx, &msg, Some(&corroborators)))
                    .allowed_mentions(CreateAllowedMentions::new())
                ).await?;
                mci.create_response(ctx, serenity::all::CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                    .content("Thank you for your input")
                    .ephemeral(true)
                )).await?;
            }
			_ => {}
		}
    }
    public_report_message.edit(ctx, EditMessage::new().components(vec![])).await?;
    Ok(())
}
