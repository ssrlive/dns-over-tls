pub use crate::args::Args;
pub use crate::error::{BoxError, Error, Result};
use crate::udp_server::UdpServer;
use std::net::{SocketAddr, UdpSocket};

mod args;
mod error;
mod udp_server;
mod upstream;

pub fn main_loop(args: &Args) -> Result<()> {
    for bind in args.bind.clone() {
        _main_loop(bind, &args)?;
    }
    Ok(())
}

pub fn _main_loop(bind: SocketAddr, args: &Args) -> Result<()> {
    log::info!("Listening for DNS requests on {}...", bind);

    let socket = UdpSocket::bind(bind)?;

    let server = UdpServer::new(&socket);

    let client = reqwest::blocking::Client::new();
    let upstreams = args.upstreams(&client);

    for request in server {
        for upstream in upstreams.iter() {
            if let Err(e) = upstream.send(&request).map(|response| server.reply(&request, &response)) {
                log::error!("error during DNS request: {:?}", e);
                continue;
            }
            break;
        }
    }
    Ok(())
}
