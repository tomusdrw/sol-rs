use types;
use bigint;

// TODO [ToDr] Throw away module after
// ethabi uses proper types.
pub mod raw {
    pub fn address(num: u64) -> [u8; 20] {
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(&*super::address(num));
        bytes
    }

    pub fn bytes32(s: &str) -> [u8; 32] {
        super::bytes32(s)
    }
}

pub fn address(num: u64) -> types::Address {
    num.into()
}

pub fn bytes32(s: &str) -> [u8; 32] {
    let bytes = s.as_bytes();
    let mut ret = [0u8; 32];
    let s = 32 - bytes.len();
    ret[s..].copy_from_slice(bytes);
    ret
}

pub fn uint(num: u64) -> types::U256 {
    num.into()
}

pub fn u256_from_bytes32(v: [u8; 32]) -> types::U256 {
    v.into()
}

pub(crate) fn convert_u256(x: types::U256) -> bigint::uint::U256 {
    let mut bytes = [0; 32];
    x.to_big_endian(&mut bytes);
    bytes.into()
}

pub(crate) fn address_to_hash(x: types::Address) -> bigint::hash::H160 {
    (&*x).into()
}
