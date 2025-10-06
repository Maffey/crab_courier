# c_c | crab_courier

Send an e-book to a predefined receiver with your Gmail account and the help of the Crab Courier!

Te main purpose of this tool is to have an easy-to-use tool for sending books to your Kindle endpoint.

## Prerequisites

Requires the following environmental variables to be configured:

- `GMAIL_USER`: Your Gmail username.
- `GMAIL_APP_PASSWORD`: Application password for your Gmail account. Can be generated here: [App Passwords](https://myaccount.google.com/apppasswords)
- `EMAIL_RECIPIENT`: The address to which send the book. Usually your custom e-mail (with `@kindle.com`).

A `.env` file can be used.c

## Usage

Right now, limited to `.epub` files.

```shell
c_c <path_to_epub>
```