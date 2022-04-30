# `cloudflare-dns-update`

Simple dynamic DNS updating for CloudFlare

## Example

Adding the following to your crontab to attempt an update every minute
```
* * * * * cloudflare-dns-update -c <API KEY> tiny-potato.xyz cinnamon.tiny-potato.xyz /tmp/my-ip.txt
```

## Installation

Prebuilt binaries are available from [GitHub releases](https://github.com/ramidzkh/cloudflare-dns-update/releases).
Prebuilt bleeding binaries are available from [GitHub Actions](https://github.com/ramidzkh/cloudflare-dns-update/actions/workflows/build.yml)

This crate can also be compiled from source
```shell
git clone https://github.com/ramidzkh/cloudflare-dns-update.git
cd cloudflare-dns-update
cargo build --release

# To install globally
sudo install target/release/cloudflare-dns-update /usr/bin/
```

## License

This project is licensed under the [MIT license](https://github.com/ramidzkh/cloudflare-dns-update/blob/master/LICENSE).
