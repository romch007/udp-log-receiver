use std::{
    fs::OpenOptions,
    io::Write,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Command {
    #[arg(short, long)]
    addr: Option<IpAddr>,

    #[arg(short, long)]
    port: u16,

    #[arg(short, long)]
    output: String,
}

fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let command = Command::parse();

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&command.output)?;

    let ip_addr = command
        .addr
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let addr = SocketAddr::new(ip_addr, command.port);

    let listener = UdpSocket::bind(addr)?;
    tracing::info!("outputting to {}", command.output);
    tracing::info!("listening on {addr}");

    loop {
        let mut buf = [0; 1 << 16];

        let (nread, peer_addr) = listener.recv_from(&mut buf)?;
        tracing::debug!("receiving from {peer_addr}");

        let buf = &buf[..nread];
        log_file.write_all(buf)?;
    }
}

fn main() {
    if let Err(error) = run() {
        tracing::error!("{error}");
    }
}
