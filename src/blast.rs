use crate::email_sender::{send_email, EmailConfig};
use crate::rss_processor::fetch_rss;
use crate::types::{BlastConfig as Config, BlastContext as Context};
use rss::Item as RSSItem;

#[derive(Debug)]
pub struct FeedResult {
    latest: RSSItem, // Assuming Item is the type of your feed items
    old: Vec<RSSItem>,
}

pub async fn fetch_feed(config: &Config) -> Option<FeedResult> {
    let feed_url = config.feed_url.as_str();
    let channel = fetch_rss(feed_url).await.expect("Failed to fetch RSS feed");
    let feed_items = channel.items().to_vec();
    if feed_items.is_empty() {
        return None;
    }
    let latest = feed_items[0].clone(); // Clone the latest item
    let old = feed_items[1..].to_vec(); // Get all other items
    Some(FeedResult { latest, old }) // Return the structured result
}

pub async fn ready_latest_item(context: &Context) -> Option<RSSItem> {
    let model = &context.model;
    let feed = fetch_feed(&context.config)
        .await
        .expect("Failed to fetch feed");

    for item in feed.old {
        match item.guid() {
            Some(item_guid) => {
                model
                    .insert_feed_item(item_guid.value(), true)
                    .await
                    .expect("Failed to insert old feed item");
            }
            None => continue,
        }
    }

    match feed.latest.guid() {
        Some(item_guid) => {
            model
                .insert_feed_item(item_guid.value(), false)
                .await
                .expect("Failed to insert latest feed item");
            let db_item = model
                .get_feed_item(item_guid.value())
                .await
                .expect("Failed to get feed item")?;
            if db_item.all_emails_sent || db_item.skip {
                return None;
            }
            Some(feed.latest)
        }
        None => None,
    }
}

fn remove_cdata_tags(text: &str) -> &str {
    let start_tag = "<![CDATA[";
    let end_tag = "]]>";

    if text.starts_with(start_tag) && text.ends_with(end_tag) {
        // Return a slice of the text excluding the CDATA tags
        &text[start_tag.len()..text.len() - end_tag.len()]
    } else {
        // Return the original string slice
        text
    }
}

pub async fn blast(context: Context) -> std::io::Result<()> {
    let model = &context.model;
    let latest = ready_latest_item(&context)
        .await
        .expect("No new posts found");
    let raw_description = latest.description().unwrap_or("No description");
    let description = remove_cdata_tags(raw_description);
    let title = latest.title().unwrap_or("No title");
    let latest_post_guid = latest.guid().expect("No guid found").value();
    let subscribers = model.get_subscribers().await.expect("no subscribers found");

    // println!("Latest post: {:?}", description);
    // println!("Title: {:?}", title);
    // println!("Guid: {:?}", latest_post_guid);
    // println!("Subscribers: {:?}", subscribers.len());

    let mut failed_emails: Vec<String> = Vec::new();

    for subscriber in subscribers {
        let subscriber_email = subscriber.email.as_str();
        let check_already_sent = context
            .model
            .check_email_sent(subscriber_email, latest_post_guid)
            .await
            .expect("Failed to check if email sent to subscriber");

        println!("subscriber_email: {:?}", subscriber_email);
        println!("check_already_sent: {:?}", check_already_sent);

        if check_already_sent {
            println!("Email already sent to {}", subscriber_email);
            continue; // Skip this subscriber
        }

        let email_config = EmailConfig {
            body: description.to_string(),
            smtp_password: context.config.smtp_pass.clone(),
            smtp_host: context.config.smtp_host.clone(),
            smtp_email: context.config.smtp_email.clone(),
            subject: title.to_string(),
            to: subscriber_email.to_string(),
        };

        println!("subject {}", email_config.subject);
        println!("to {}", email_config.to);

        match send_email(email_config) {
            Ok(_) => {
                println!("Email sent successfully to {}", subscriber_email);
                context
                    .model
                    .mark_email_as_sent(subscriber_email, latest_post_guid)
                    .await
                    .expect("Failed to mark email as sent");
            }
            Err(e) => {
                println!("Failed to send email to {}: {:?}", subscriber_email, e);
                context
                    .model
                    .mark_email_as_failure(subscriber_email, latest_post_guid)
                    .await
                    .expect("Failed to mark send failure");
                failed_emails.push(subscriber_email.to_string());
            }
        }
    }

    // Check if there were any failures
    if failed_emails.is_empty() {
        println!("Success 100%: All emails sent successfully.");
        model
            .mark_all_emails_sent(latest_post_guid)
            .await
            .expect("Failed to mark all emails as sent");
    }

    Ok(())
}
