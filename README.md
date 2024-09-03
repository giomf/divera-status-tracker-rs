# Command-Line Help for `dst`

This document contains the help content for the `dst` command-line program.

**Command Overview:**

* [`dst`↴](#dst)
* [`dst fetch`↴](#dst-fetch)
* [`dst aggregate`↴](#dst-aggregate)
* [`dst update`↴](#dst-update)
* [`dst on-duty`↴](#dst-on-duty)
* [`dst history`↴](#dst-history)

## `dst`

Divera Status Tracker (dst)

**Usage:** `dst <COMMAND>`

###### **Subcommands:**

* `fetch` — Fetch all attachments
* `aggregate` — Aggregate attachments
* `update` — Fetch all attachments and aggregate them
* `on-duty` — Create on-duty table
* `history` — Create history plot



## `dst fetch`

Fetch all attachments

**Usage:** `dst fetch --email <EMAIL> --host <HOST> --password <PASSWORD> --subject <SUBJECT>`

###### **Options:**

* `--email <EMAIL>` — The email address used to fetch the attachments
* `--host <HOST>` — The host used to connect to
* `--password <PASSWORD>` — The password that matches the email
* `--subject <SUBJECT>` — The email subject to filter the attachments



## `dst aggregate`

Aggregate attachments

**Usage:** `dst aggregate --off-duty-keyword <OFF_DUTY_KEYWORD>`

###### **Options:**

* `--off-duty-keyword <OFF_DUTY_KEYWORD>` — The keyword which is used in divera to indicate the off-duty status



## `dst update`

Fetch all attachments and aggregate them

**Usage:** `dst update --email <EMAIL> --host <HOST> --password <PASSWORD> --subject <SUBJECT> --off-duty-keyword <OFF_DUTY_KEYWORD>`

###### **Options:**

* `--email <EMAIL>` — The email address used to fetch the attachments
* `--host <HOST>` — The host used to connect to
* `--password <PASSWORD>` — The password that matches the email
* `--subject <SUBJECT>`
* `--off-duty-keyword <OFF_DUTY_KEYWORD>` — The keyword which is used in divera to indicate the off-duty status



## `dst on-duty`

Create on-duty table

**Usage:** `dst on-duty [OPTIONS]`

###### **Options:**

* `--print` — Print the overview
* `--export` — Export the overview as xlsx



## `dst history`

Create history plot

**Usage:** `dst history [OPTIONS]`

###### **Options:**

* `--print` — Print the overview
* `--export` — Export the overview as html
* `--year <YEAR>` — The year
* `--month <MONTH>` — The month



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
