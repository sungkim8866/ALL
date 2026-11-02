//! Finds other DeskLink computers on the local network with UDP broadcasts.

use std::collections::HashSet;
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use tokio::net::UdpSocket;
use tokio::sync::mpsc::UnboundedSender;

use crate::proto::{Beacon, BEACON_MAGIC, PROTOCOL_VERSION};

#[derive(Clone, Debug)]
pub struct Seen {
    pub beacon: Beacon,
    pub addr: SocketAddr,
}

fn broadcast_targets(port: u16) -> Vec<SocketAddr> {
    let mut set: HashSet<Ipv4Addr> = HashSet::new();
    set.insert(Ipv4Addr::BROADCAST);
    if let Ok(ifaces) = if_addrs::get_if_addrs() {
        for iface in ifaces {
            if iface.is_loopback() {
                continue;
            }
            if let if_addrs::IfAddr::V4(v4) = iface.addr {
                if let Some(b) = v4.broadcast {
                    set.insert(b);
                } else {
                    let ip = u32::from(v4.ip);
                    let mask = u32::from(v4.netmask);
                    set.insert(Ipv4Addr::from(ip | !mask));
                }
            }
        }
    }
    set.into_iter().map(|ip| SocketAddr::from((ip, port))).collect()
}

/// Binds the discovery port so that several sockets may share it (another
/// copy of DeskLink, or one that hasn't exited yet, must not stop discovery).
fn bind_shared(port: u16) -> std::io::Result<UdpSocket> {
    use socket2::{Domain, Protocol, Socket, Type};
    let s = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    s.set_reuse_address(true)?;
    #[cfg(all(unix, not(target_os = "solaris"), not(target_os = "illumos")))]
    s.set_reuse_port(true)?;
    s.set_broadcast(true)?;
    s.set_nonblocking(true)?;
    s.bind(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)).into())?;
    UdpSocket::from_std(s.into())
}

/// Announces this computer every two seconds and reports beacons from others.
/// `make_beacon` is called before each announcement so name changes show up.
pub async fn run(
    port: u16,
    make_beacon: impl Fn() -> Beacon + Send + 'static,
    tx: UnboundedSender<Seen>,
) -> anyhow::Result<()> {
    let sock = bind_shared(port)?;
    let mut buf = vec![0u8; 2048];
    let mut tick = tokio::time::interval(Duration::from_secs(2));
    loop {
        tokio::select! {
            _ = tick.tick() => {
                let b = make_beacon();
                let bytes = postcard::to_stdvec(&b)?;
                for target in broadcast_targets(port) {
                    // Errors here are normal (interfaces going up and down).
                    let _ = sock.send_to(&bytes, target).await;
                }
            }
            r = sock.recv_from(&mut buf) => {
                let (n, from) = match r {
                    Ok(v) => v,
                    Err(_) => continue, // e.g. ICMP port unreachable on Windows
                };
                if let Ok(beacon) = postcard::from_bytes::<Beacon>(&buf[..n]) {
                    if beacon.magic == BEACON_MAGIC && beacon.version == PROTOCOL_VERSION {
                        let addr = SocketAddr::new(from.ip(), beacon.port);
                        if tx.send(Seen { beacon, addr }).is_err() {
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
}
