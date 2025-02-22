// LNP/BP Core Library implementing LNPBP specifications & standards
// Written in 2019 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use internet2::addr::{
    AddrParseError, NodeAddr, NodeAddrParseError, PartialNodeAddr,
};
use p2p::bifrost::LNP2P_BIFROST_PORT;
use p2p::bolt::LNP2P_BOLT_PORT;
use p2p::Protocol;

/// LNP node address containing both node address and the used protocol.
/// When parsed from string or displayed, it may omit port information and use
/// the protocol default port.
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
#[derive(NetworkEncode, NetworkDecode)]
pub struct LnpAddr {
    /// Protocol used for connection.
    pub protocol: Protocol,

    /// Remote peer address for connecting to.
    pub node_addr: NodeAddr,
}

impl LnpAddr {
    /// Construct BOLT-compatible node address.
    pub fn bolt(addr: PartialNodeAddr) -> LnpAddr {
        LnpAddr {
            protocol: Protocol::Bolt,
            node_addr: addr.node_addr(LNP2P_BOLT_PORT),
        }
    }

    /// Construct Bifrost-compatible node address.
    pub fn bifrost(addr: PartialNodeAddr) -> LnpAddr {
        LnpAddr {
            protocol: Protocol::Bifrost,
            node_addr: addr.node_addr(LNP2P_BIFROST_PORT),
        }
    }

    /// Returns used port address.
    pub fn port(self) -> u16 {
        self.node_addr
            .addr
            .port()
            .expect("port information always present in NodeAddr")
    }
}

impl Display for LnpAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}://{}@{}",
            self.protocol,
            self.node_addr.id,
            self.node_addr.addr.address()
        )?;
        if self.protocol.default_port() != self.port() {
            write!(f, ":{}", self.port())?;
        }
        Ok(())
    }
}

impl FromStr for LnpAddr {
    type Err = NodeAddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("://");
        match (
            split.next().map(str::to_lowercase).as_deref(),
            split.next(),
            split.next(),
        ) {
            (Some("bolt"), Some(addr), None) => {
                PartialNodeAddr::from_str(addr).map(LnpAddr::bolt)
            }
            (Some("bifrost"), Some(addr), None) => {
                PartialNodeAddr::from_str(addr).map(LnpAddr::bifrost)
            }
            (Some(unknown), ..) => {
                Err(AddrParseError::UnknownProtocolError(unknown.to_owned())
                    .into())
            }
            _ => Err(AddrParseError::WrongAddrFormat(s.to_owned()).into()),
        }
    }
}
