extern crate digest;
extern crate log;
extern crate proptest;
extern crate tari_mmr;
extern crate tari_crypto;

use log::{info, trace, warn};

pub mod common;
pub mod tari_core_mmr;
pub mod tari_util;

/// Tari Core targets
pub use tari_core_mmr::*;


/// Tari Util targets
pub use tari_util::*;

/// Tari Crypto targets

// Targets
const TARGETS: &[&str] = &[
   "fuzz_mmr_push_bytes",
   "util_to_hex",
   "util_bytes_to_bits",
   "util_from_hex",
   "util_to_hex_multiple",
];


pub fn list_targets() {
    trace!("Getting a list of targets");
    for target in &*TARGETS {
        println!("\u{1F489} {:?}", target)
    }
}

pub fn check_target(target: &str) {
    trace!("Checking if the supplied target is available");
    if TARGETS.iter().any(|&x| x == target) {
        info!("Target located!");
    }
    else {

        warn!("Did not find the target: {:?}", &target);
        panic!("Uknkown fuzz target {} selected", target);

    }
}
