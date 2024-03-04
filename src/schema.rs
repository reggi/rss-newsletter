
const FEED_ITEMS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS feed_items (
    feed_url TEXT NOT NULL,
    item_guid TEXT NOT NULL,
    all_emails_sent BOOLEAN NOT NULL DEFAULT 0,
    skip BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (item_guid)
)
";

const SUBSCRIBERS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS subscribers (
    feed_url TEXT NOT NULL,
    email TEXT NOT NULL,
    unsubscribe BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (email)
)
";

const EMAILS_SENT_TABLE: &str = "
CREATE TABLE IF NOT EXISTS emails_sent (
    item_guid TEXT NOT NULL,
    email TEXT NOT NULL,
    PRIMARY KEY (item_guid, email),
    FOREIGN KEY (item_guid) REFERENCES emails (item_guid)
        ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (email) REFERENCES subscribers (email)
        ON DELETE CASCADE ON UPDATE NO ACTION
)
";
