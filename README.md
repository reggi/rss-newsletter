# `rss-newsletter` (`rssn`)

* The goal of this project is to take an RSS feed and turn it into a newsletter.
* This uses `smtp` to send emails through your account, can use `gmail` or `Google Workspace`.
* Contains a server with two forms for `subscribe` / `unsubscribe`.
* Uses `sqlite` to store subscriber / which emails have been sent to whom.
* Containts a `script` for cron to send daily or weekly emails, "latest only".

```bash
Sends newsletter updates from an RSS feed

Usage: rssn [OPTIONS] --feed <URL> --sqlite-file <FILE_PATH>

Options:
      --feed <URL>               The file name of the SQLite database [env: FEED=]
      --sqlite-file <FILE_PATH>  The file name of the SQLite database [env: SQLITE_FILE=]
      --smtp-host <HOSTNAME>     Hostname of the SMTP server [env: SMTP_HOST=] [default: smtp.gmail.com]
      --smtp-port <INT>          Port for the SMTP server [env: SMTP_PORT=] [default: 587]
      --smtp-email <EMAIL>       Email address to send email from [env: SMTP_EMAIL=]
      --smtp-pass <STRING>       Password for the SMTP email address [env: SMTP_PASS=]
      --port <INT>               A port for the server [env: PORT=] [default: 80]
      --action <serve|blast>     Action to perform: `serve` or `blast` [env: ACTION=] [possible values: serve, blast]
  -h, --help                     Print help
  -V, --version                  Print version
```