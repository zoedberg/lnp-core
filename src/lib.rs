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

#![recursion_limit = "256"]
// Coding conventions
#![deny(dead_code, /* missing_docs, */ warnings)]

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_with;
#[cfg(feature = "serde")]
extern crate serde_crate as serde;

pub extern crate lnp2p as p2p;

macro_rules! dumb_pubkey {
    () => {
        secp256k1::PublicKey::from_secret_key(
            secp256k1::SECP256K1,
            &secp256k1::ONE_KEY,
        )
    };
}

pub mod addr;
pub mod channel;
pub mod extension;
pub mod router;

pub use channel::Channel;
pub use extension::{
    ChannelConstructor, ChannelExtension, Extension, RouterExtension,
};
