// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde_derive::Serialize;
use std::path;

#[derive(Debug, Serialize, Clone)]
pub struct NetConfig {
    pub port: u16,

    pub grpc_port: u16,

    pub peers: Vec<PeerConfig>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PeerConfig {
    pub address: String,
}

impl NetConfig {
    pub fn new(
        port: u16,
        grpc_port: u16,
        addresses: Vec<String>
    ) -> Self {
        let mut peers = Vec::with_capacity(addresses.len());
        for address in addresses {
            peers.push(PeerConfig{
                address,
            })
        }
        Self {
            port,
            grpc_port,
            peers
        }
    }

    pub fn write_to_file(&self, path: impl AsRef<path::Path>) {
        crate::util::write_to_file(&self, path, "network_p2p".to_string());
    }
}

#[cfg(test)]
mod network_p2p_test {
    use super::*;
    use toml::Value;
    use crate::util::write_to_file;

    #[test]
    fn basic_test() {
        let _ = std::fs::remove_file("example/config.toml");

        let peers = vec!["/ip4/127.0.0.1/tcp/40001".to_string(), "/ip4/127.0.0.1/tcp/40002".to_string(), "/ip4/127.0.0.1/tcp/40003".to_string()];
        let config = NetConfig::new(
            51230,
            40000,
            peers,
        );

        config.write_to_file("example/config.toml");
    }
}

