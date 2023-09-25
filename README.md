# `cloudflare-dns-update`

Simple dynamic DNS updating for CloudFlare

## Example

Adding the following to your crontab to attempt an update every minute
```
* * * * * cloudflare-dns-update --credentials <API KEY> --cache /tmp/my-ipv4.txt de238d165c115286820c8e0852cd325c server.example.com
```

You can use an argsfile, using `@file.txt`. Use to hide credentials from coming up in `ps a` or cron log

## Installation

This crate can also be compiled from source
```shell
git clone https://github.com/ramidzkh/cloudflare-dns-update.git
cd cloudflare-dns-update
cargo build --release

# To install globally
sudo install target/release/cloudflare-dns-update /usr/local/bin/
```

## License

This project is licensed under the [MIT license](https://github.com/ramidzkh/cloudflare-dns-update/blob/master/LICENSE).
