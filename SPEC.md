## Using a RSS Feed as a Newsletter

I'm looking to create a `rust` server that uses a `sqlite` db.

The goals of the progam are as follows:

* The script would be called `rssnewsletter`.
* The goal is to email a list of people updates from an rss feed.
* The script fetches rss feed and caches which posts have been sent out.
* Initially when the script is first run and the database is fresh we just mark all recent posts as already `skip`.

The database tables would look like this:

The `emails` table looks like this:

* `feed` - string - the url of the feed, same as `--feed`
* `item_guid` string - the permalink / guid of an individual rss item entry
* `sent` - boolean - wheather this post has been sent out or not
* `skip` - boolean - entries marked as skipped are old posts that were in the feed before the server was initiated

The `subscribers` table looks like this:

* `email` - string - the email of the subscriber
* `unsubscribe` - boolean - if the user has manaually opted out via the API set to true

Here are the flags and what they do:

* `--feed` - a url of the rss feed you wish to use as a newsletter
* `--smtp-host` [optional] - hostname of the smtp server (default: `smtp.gmail.com`)
* `--smtp-port` [optional] - a port for the smtp server (default: `587`)
* `--smtp-email` - a email address to send email from
* `--smtp-pass` - the password to the email address
* `--sqlite-file` - the file name of the sqlite database
* `--port` [optional] a port for the server (default: `80`)
* `--action` [optional] - enumerable string either `serve` or `blast` (default: `serve`)
* `--subscribe-html` [optional] - a path to a file for an HTML template for subscription, this allows users to customize the html for the form
* `--unsubscribe-html` [optional] - a path to a file for an HTML template for unsubscribe, this allows users to customize the html for the form
  * `serve` will spin up a server running on the port assigned 
    * `GET` - `/api/unsubscribe&email=example@email.com` 
      * removes email from `emails` table
    * `GET` - `/api/subscribe&email=example@email.com` 
      * adds email to `emails` table
    * `GET` - `/unsubscribe` 
      * displays HTML form with action pointing to `/api/unsubscribe`
      * also accept `&email=example@email.com` to pre-populate form
    * `GET` - `/subscribe` 
      * displays HTML form with action pointing to `/api/unsubscribe`
      * also accept `&email=example@email.com` to pre-populate form
  * `blast` - this action will do the following:
    * fetch the rss feed
    * using the items `guid` ensure all of the `items` in the feed are in the database
    * if multiple "new" items are found, we plan to send the latest one, mark all others as `skip`
    * fetch all `subscribers` where `unsubscribe` is set to `false`
    * loop over every `subscriber` and send the email
    * the bottom of the email should contain a `unsubscribe` link that links to the email for this subscriber
* `--send-latest-if-skipped` [optional] - specifically for the `--action blast` will send the latest email even if it's been skipped.