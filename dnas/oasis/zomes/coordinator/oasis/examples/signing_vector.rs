use hdk::prelude::*;
use holochain_nonce::Nonce256Bits;

fn hash_bytes(prefix: [u8; 3], start: u8) -> Vec<u8> {
    let mut value = prefix.to_vec();
    value.extend((0..32).map(|i| start.wrapping_add(i)));
    value.extend([0, 0, 0, 0]);
    value
}

fn main() {
    let call = ZomeCallParams {
        provenance: AgentPubKey::from_raw_39(hash_bytes([0x84, 0x20, 0x24], 1)),
        cell_id: CellId::new(
            DnaHash::from_raw_39(hash_bytes([0x84, 0x2d, 0x24], 41)),
            AgentPubKey::from_raw_39(hash_bytes([0x84, 0x20, 0x24], 81)),
        ),
        zome_name: ZomeName::from("oasis"),
        fn_name: FunctionName::from("apply_hyperdrive_mutation"),
        cap_secret: Some(CapSecret::from([121u8; 64])),
        payload: ExternIO::from((0..73).map(|i| 5u8.wrapping_add(i)).collect::<Vec<_>>()),
        nonce: Nonce256Bits::from([77u8; 32]),
        expires_at: Timestamp::from_micros(1_765_432_100_000_000),
    };
    let (bytes, hash) = call.serialize_and_hash().expect("signing vector");
    println!("bytes={}", bytes.iter().map(|b| format!("{b:02x}")).collect::<String>());
    println!("hash={}", hash.iter().map(|b| format!("{b:02x}")).collect::<String>());
}
