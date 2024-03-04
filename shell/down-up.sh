source .env
rm -rf ./newsletter.sqlite
rm -rf ./newsletter.sqlite-shm
rm -rf ./newsletter.sqlite-wal
sh ./shell/init.sh
sh ./shell/subscribe.sh --email $GUINEA_PIG_EMAIL