use std::time::Duration;

use async_std::io::{timeout as tout, Error};
use async_std::net::TcpStream;

pub async fn check_network(addrs: &str, timeout: Duration) -> Result<bool, Error> {
    match tout(timeout, TcpStream::connect(addrs)).await {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}
