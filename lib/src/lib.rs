// lib.rs
use uint::construct_uint;
construct_uint! {
    // Construct a unsigned 256-bit integer
    // consisting of 4 x 64 bit words
    pub struct U256(4);
}
pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;
