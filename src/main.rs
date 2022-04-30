use std::fs::File;
use std::io::{Read, Write};
use std::net::Ipv4Addr;
use std::str::FromStr;

use clap::Parser;
use cloudflare::endpoints::dns::{
    DnsContent, ListDnsRecords, ListDnsRecordsParams, UpdateDnsRecord, UpdateDnsRecordParams,
};
use cloudflare::endpoints::zone::{ListZones, ListZonesParams};

use crate::cli::Cli;

mod cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse_from(
        argfile::expand_args_from(wild::args_os(), argfile::parse_fromfile, argfile::PREFIX)
            .unwrap(),
    );
    let client = cli.client.create_client();

    let (ip, content) = {
        let response = reqwest::get("https://checkip.amazonaws.com/")
            .await
            .unwrap();
        let ip = response.text().await.unwrap();

        // Compare with temporary file
        if let Some(old_value) = cli
            .cache
            .as_ref()
            .and_then(|location| File::open(location).ok())
            .and_then(|mut file| {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).map(|_| buffer).ok()
            })
        {
            if old_value == ip {
                return;
            }
        }

        let address = Ipv4Addr::from_str(ip.trim()).unwrap();
        (ip, address)
    };

    let zone = client
        .request_handle(&ListZones {
            params: ListZonesParams {
                name: Some(cli.zone),
                ..Default::default()
            },
        })
        .await
        .unwrap()
        .result
        .into_iter()
        .next()
        .expect("Zone could not be found")
        .id;

    let record = client
        .request_handle(&ListDnsRecords {
            zone_identifier: &zone,
            params: ListDnsRecordsParams {
                name: Some(cli.record.clone()),
                ..Default::default()
            },
        })
        .await
        .unwrap()
        .result
        .into_iter()
        .next()
        .expect("Record could not be found");

    client
        .request_handle(&UpdateDnsRecord {
            zone_identifier: &zone,
            identifier: &record.id,
            params: UpdateDnsRecordParams {
                ttl: Some(record.ttl),
                proxied: Some(record.proxied),
                name: &cli.record,
                content: DnsContent::A { content },
            },
        })
        .await
        .unwrap();

    // Save in temporary file
    if let Some(mut file) = cli.cache.and_then(|location| File::create(location).ok()) {
        #[allow(unused_must_use)]
        {
            write!(file, "{}", ip);
        }
    }
}
