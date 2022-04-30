use std::str::FromStr;

use clap::{Args, Error, Parser};
use cloudflare::framework::async_api::Client as CfClient;
use cloudflare::framework::auth::Credentials as CfCredentials;
use cloudflare::framework::Environment;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Cli {
    #[clap(flatten)]
    pub client: Client,

    /// Zone to search the record in, such as example.org
    pub zone: String,
    /// Record to search for, such as www.example.org
    pub record: String,

    /// Cache for the IP, so that CloudFlare servers are not unnecessarily pinged
    pub cache: Option<String>,
}

#[derive(Args, Debug)]
pub struct Client {
    /// Your CloudFlare authentication {n}
    ///  * For email and API key, use <email>=<api key> {n}
    ///  * For service token, use service=<token> {n}
    ///  * For user token, use <token> {n}
    #[clap(short, long)]
    credentials: Option<Credentials>,
}

impl Client {
    pub fn create_client(self) -> CfClient {
        CfClient::new(
            self.credentials
                .expect("Credentials are required for this operation")
                .0,
            Default::default(),
            Environment::Production,
        )
        .unwrap()
    }
}

#[derive(Debug)]
pub struct Credentials(pub CfCredentials);

impl FromStr for Credentials {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("=") {
            Some(("service", key)) => Ok(Self(CfCredentials::Service {
                key: key.to_string(),
            })),
            Some((email, key)) => Ok(Self(CfCredentials::UserAuthKey {
                email: email.to_string(),
                key: key.to_string(),
            })),
            None => Ok(Self(CfCredentials::UserAuthToken {
                token: s.to_string(),
            })),
        }
    }
}
