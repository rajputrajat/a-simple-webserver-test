//#![feature(addr_parse_ascii)]

use anyhow::Result;
use futures::{future, StreamExt};
use rig_info::RigInfo;
use std::{env, net::Ipv4Addr};
use tarpc::{
    context::Context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Json,
};
use tokio::process::Command;

#[derive(Clone)]
struct RigServer;

#[tarpc::server]
impl RigInfo for RigServer {
    async fn ip(self, _: Context) -> String {
        let out = Command::new("ps").output().await.unwrap().stdout;
        String::from_utf8(out).unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // let args = env::args().skip(1).collect::<Vec<_>>();
    // let (ip, port) = (&args[0], &args[1]);
    //let ip = Ipv4Addr::parse_ascii(ip.as_bytes()).unwrap();
    let ip: Ipv4Addr = "172.26.8.90".parse().unwrap();
    let mut listner =
        //tarpc::serde_transport::tcp::listen((ip, port.parse::<u16>()?), Json::default).await?;
        tarpc::serde_transport::tcp::listen((ip, 3001), Json::default).await?;
    listner.config_mut().max_frame_length(usize::MAX);
    listner
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = RigServer;
            channel.execute(server.serve())
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;
    Ok(())
}
