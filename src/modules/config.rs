use std::{
    net::{SocketAddr, ToSocketAddrs},
    path::PathBuf,
};

#[derive(Debug)]
pub struct Config {
    pub addr: SocketAddr,
    pub memory_limit: usize,
    pub memory_free_size: usize,
    pub work_path: PathBuf,
}

impl Config {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:7595".to_socket_addrs().unwrap().as_ref()[0],
            memory_limit: 256 * 1024 * 1024,
            memory_free_size: 1 * 1024 * 1024,
            work_path: PathBuf::new(),
        }
    }
}

pub fn load() -> Config {
    Config::default()
}
