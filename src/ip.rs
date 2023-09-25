use tokio::select;

pub async fn find_ip(ipv6: bool) -> Option<String> {
    if ipv6 {
        find_ipv6().await
    } else {
        find_ipv4().await
    }
}

async fn find_ipv4() -> Option<String> {
    select! {
        Ok(ip) = amazon() => Some(ip),
        Ok(ip) = ident_ipv4() => Some(ip),
        else => None,
    }
}

async fn find_ipv6() -> Option<String> {
    select! {
        Ok(ip) = ident_ipv6() => Some(ip),
        else => None,
    }
}

async fn amazon() -> reqwest::Result<String> {
    reqwest::get("https://checkip.amazonaws.com/")
        .await?
        .text()
        .await
        .map(trim)
}

async fn ident_ipv4() -> reqwest::Result<String> {
    reqwest::get("https://v4.ident.me/")
        .await?
        .text()
        .await
        .map(trim)
}

async fn ident_ipv6() -> reqwest::Result<String> {
    reqwest::get("https://v6.ident.me/")
        .await?
        .text()
        .await
        .map(trim)
}

fn trim(string: String) -> String {
    string.trim().to_string()
}

#[tokio::test]
async fn test() {
    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::str::FromStr;

    if let Ok(ip) = &amazon().await {
        assert!(Ipv4Addr::from_str(ip).is_ok());
    }

    if let Ok(ip) = &ident_ipv4().await {
        assert!(Ipv4Addr::from_str(ip).is_ok());
    }

    if let Ok(ip) = &ident_ipv6().await {
        assert!(Ipv6Addr::from_str(ip).is_ok());
    }
}
