#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
    export $(cat .env | sed 's/#.*//g' | xargs)
else
    echo ".env file not found"
    exit 1
fi

# Email content and recipient
RECIPIENT="$GUINEA_PIG_EMAIL" # Change this to the recipient's email address
SUBJECT="SMTP Test Email"
BODY="This is a test email sent via SMTP using curl."

# Send the email
curl -v --ssl-reqd \
     --url "smtp://$SMTP_HOST:$SMTP_PORT" \
     --user "$SMTP_EMAIL:$SMTP_PASS" \
     --mail-from "$SMTP_EMAIL" \
     --mail-rcpt "$RECIPIENT" \
     --upload-file - <<EOF
From: $SMTP_EMAIL
To: $RECIPIENT
Subject: $SUBJECT

$BODY
EOF

if [ $? -eq 0 ]; then
    echo "Email sent successfully"
else
    echo "Failed to send email"
fi
