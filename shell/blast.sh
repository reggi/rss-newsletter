source .env

cargo run -- \
--feed="$FEED" \
--smtp-email="$SMTP_EMAIL" \
--smtp-pass="$SMTP_PASS" \
--smtp-host="$SMTP_HOST" \
--smtp-port="$SMTP_PORT" \
--sqlite-file="$SQLITE_FILE" \
--action="blast"
