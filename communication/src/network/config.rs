use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use time::UTime;

/// Network configuration
#[derive(Debug, Deserialize, Clone)]
pub struct NetworkConfig {
    /// Where to listen for communications.
    pub bind: SocketAddr,
    /// Our own IP if it is routable, else None.
    pub routable_ip: Option<IpAddr>,
    /// Protocol port
    pub protocol_port: u16,
    /// Time intervall spent waiting for a response from a peer.
    /// In millis
    pub connect_timeout: UTime,
    /// Network_worker will try to connect to avaible peers every wakeup_interval.
    /// In millis
    pub wakeup_interval: UTime,
    /// Path to the file containing knos=wn peers.
    pub peers_file: std::path::PathBuf,
    /// Number of avaible slots for out connections.
    pub target_out_connections: usize,
    /// Limit on the number of in connections.
    pub max_in_connections: usize,
    /// Limit on the number of in connections per ip.
    pub max_in_connections_per_ip: usize,
    /// Limit on the total current number of out connection attempts.
    pub max_out_connnection_attempts: usize,
    /// Limit on the number of idle peers we remember.
    pub max_idle_peers: usize,
    /// Limit on the number of banned peers we remember.
    pub max_banned_peers: usize,
    /// Limit on the number of peers we advertize to others.
    pub max_advertise_length: usize,
    /// Peer database is dumped every peers_file_dump_interval.
    /// In millis.
    pub peers_file_dump_interval: UTime,
}