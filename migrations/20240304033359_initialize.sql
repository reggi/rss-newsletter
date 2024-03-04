CREATE TABLE IF NOT EXISTS feed_items (
    feed_url TEXT NOT NULL,
    item_guid TEXT NOT NULL,
    all_emails_sent BOOLEAN NOT NULL DEFAULT 0,
    skip BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (item_guid)
);

CREATE TABLE IF NOT EXISTS subscribers (
    feed_url TEXT NOT NULL,
    email TEXT NOT NULL,
    unsubscribe BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (email, feed_url)
);

CREATE TABLE IF NOT EXISTS emails_sent (
    feed_url TEXT NOT NULL,
    item_guid TEXT NOT NULL,
    email TEXT NOT NULL,
    success BOOLEAN NOT NULL DEFAULT 0,
    failures INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (feed_url, item_guid, email),
    FOREIGN KEY (item_guid) REFERENCES feed_items (item_guid),
    FOREIGN KEY (email, feed_url) REFERENCES subscribers (email, feed_url)
);
