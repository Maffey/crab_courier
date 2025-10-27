# c_c | crab_courier

Send an e-book to a predefined receiver with your Gmail account and the help of the Crab Courier!

Te main purpose of this tool is to have an easy-to-use tool for sending books to your Kindle endpoint.

## Prerequisites

Requires the Gmail [App Password](https://myaccount.google.com/apppasswords) to be generated.

The credentials and email recipient can be provided either by CLI arguments or using environmental variables.

### Environmental variables

- `GMAIL_USERNAME`: Your Gmail username.
- `GMAIL_APP_PASSWORD`: Application password for your Gmail account.
- `EMAIL_RECIPIENT`: The address to which send the book. Usually your custom e-mail (with `@kindle.com`).

A `.env` file is supported. Must be in the same location as application binary.

## Usage

Right now, limited to `.epub` files, although it's likely other file types will work as well.

```shell
crab_courier <path_to_epub>
```