# `rss-newsletter`

* The goal of this project is to take an RSS feed and turn it into a newsletter.
* This uses `smtp` to send emails through your account, can use `gmail` or `Google Workspace`.
* Contains a server with two forms for `subscribe` / `unsubscribe`.
* Uses `sqlite` to store subscriber / which emails have been sent to whom.
* Containts a `script` for cron to send daily or weekly emails, "latest only".
* You can use `env` vars instead of flags.

```bash
Manages subscriptions and sends updates

Usage: rss-newsletter [COMMAND]

Commands:
  blast       Sends the latest RSS post to all subscribers
  server      Starts the server for subscribing/unsubscribing to the newsletter
  subscriber  Interact with a subscriber.
  test-email  Send a test email to the provided email address.
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```bash
Sends the latest RSS post to all subscribers

Usage: rss-newsletter blast [OPTIONS] --sqlite-file <FILE_PATH> --feed-url <URL>

Options:
      --sqlite-file <FILE_PATH>  The file name of the SQLite database [env: SQLITE_FILE=]
      --smtp-host <HOSTNAME>     Hostname of the SMTP server [env: SMTP_HOST=] [default: smtp.gmail.com]
      --smtp-port <INT>          Port for the SMTP server [env: SMTP_PORT=] [default: 587]
      --smtp-email <EMAIL>       Email address to send email from [env: SMTP_EMAIL=]
      --smtp-pass <STRING>       Password for the SMTP email address [env: SMTP_PASS=]
  -f, --feed-url <URL>           The url of the rss feed [env: FEED_URL=]
  -h, --help                     Print help
```

```bash
Starts the server for subscribing/unsubscribing to the newsletter

Usage: rss-newsletter server [OPTIONS] --sqlite-file <FILE_PATH> --feed-url <URL>

Options:
      --sqlite-file <FILE_PATH>  The file name of the SQLite database [env: SQLITE_FILE=]
      --port <INT>               A port for the server [env: PORT=] [default: 80]
  -f, --feed-url <URL>           The url of the rss feed [env: FEED_URL=]
  -h, --help                     Print help
```

```bash
Interact with a subscriber.

Usage: rss-newsletter subscriber [OPTIONS] --feed-url <URL> --sqlite-file <FILE_PATH> --email <EMAIL>

Options:
  -f, --feed-url <URL>           The url of the rss feed [env: FEED_URL=]
      --sqlite-file <FILE_PATH>  The file name of the SQLite database [env: SQLITE_FILE=]
  -u, --unsubscribe              Unsubscribe the user from the feed
  -e, --email <EMAIL>            The email address to interact with
  -h, --help                     Print help
```

```bash
Send a test email to the provided email address.

Usage: rss-newsletter test-email [OPTIONS] --email <EMAIL>

Options:
      --smtp-host <HOSTNAME>  Hostname of the SMTP server [env: SMTP_HOST=] [default: smtp.gmail.com]
      --smtp-port <INT>       Port for the SMTP server [env: SMTP_PORT=] [default: 587]
      --smtp-email <EMAIL>    Email address to send email from [env: SMTP_EMAIL=]
      --smtp-pass <STRING>    Password for the SMTP email address [env: SMTP_PASS=]
  -e, --email <EMAIL>         The email address to interact with
  -h, --help                  Print help
```
