//! ONET over Holochain: OASIS ONET nodes find each other and exchange ONET application frames through this DNA.
//!
//! A node publishes its signed ONET node record (ONET node id, P-256 identity public key, advertised address,
//! signature) as its membership in the HyperNET session `onet`, so `onet_nodes` is discovery. Frames are already
//! encrypted for the target and signed by the sender's ONET identity (ONODE `ONETFrameCrypto`), so the zome carries
//! them as opaque bytes in a HyperNET remote signal of kind `onet-frame`; the receiving ONODE verifies and decrypts.
//! The record signature and identity are verified by ONODE clients, which hold the ONET key formats.
//! Coordinator-only, so the DNA hash (and existing data) is unchanged.

use hdk::prelude::*;

use crate::hypernet::{peers, send_to, session_base, session_links, signal, HyperNETPeer};
use oasis_integrity::*;

pub const ONET_SESSION: &str = "onet";
pub const ONET_FRAME_KIND: &str = "onet-frame";

/// Link tags are bounded by Holochain; a node record must fit in one.
const MAX_NODE_RECORD_BYTES: usize = 900;
/// ONODE caps application frames at 4 MiB; remote signals are kept well below the network message limit.
const MAX_FRAME_BYTES: usize = 512 * 1024;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ONETNodeRecord {
    pub node_id: String,
    pub public_key: String,
    #[serde(default)]
    pub advertised_address: String,
    /// The node's ONET identity signature over `onet.node|<node_id>|<agent>|<advertised_address>`.
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ONETNode {
    pub agent: AgentPubKey,
    pub record: ONETNodeRecord,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ONETSendInput {
    pub to: AgentPubKey,
    pub frame: serde_bytes::ByteBuf,
}

fn invalid(message: &str) -> WasmError {
    wasm_error!(WasmErrorInner::Guest(message.into()))
}

pub(crate) fn validate_record(record: &ONETNodeRecord) -> Result<String, String> {
    let is_hex_sha256 = record.node_id.len() == 64 && record.node_id.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase());
    if !is_hex_sha256 {
        return Err("ONET node id must be the lowercase hex SHA-256 of its public key".into());
    }
    if record.public_key.trim().is_empty() || record.signature.trim().is_empty() {
        return Err("ONET node record requires a public key and signature".into());
    }
    let json = serde_json::to_string(record).map_err(|e| e.to_string())?;
    if json.len() > MAX_NODE_RECORD_BYTES {
        return Err(format!("ONET node record is {} bytes; the limit is {MAX_NODE_RECORD_BYTES}", json.len()));
    }
    Ok(json)
}

/// Publishes (or replaces) this agent's ONET node record; returns every registered ONET node, including this one.
#[hdk_extern]
pub fn onet_register_node(record: ONETNodeRecord) -> ExternResult<Vec<ONETNode>> {
    let json = validate_record(&record).map_err(|e| invalid(&e))?;
    let me = agent_info()?.agent_initial_pubkey;
    for link in session_links(ONET_SESSION)? {
        if AgentPubKey::try_from(link.target.clone()).map(|a| a == me).unwrap_or(false) {
            delete_link(link.create_link_hash, GetOptions::default())?;
        }
    }
    create_link(session_base(ONET_SESSION)?, me, LinkTypes::AllAvatars, LinkTag::new(json.into_bytes()))?;
    onet_nodes(())
}

/// Every agent with a well-formed ONET node record (records that do not parse or validate are skipped).
#[hdk_extern]
pub fn onet_nodes(_: ()) -> ExternResult<Vec<ONETNode>> {
    Ok(peers(ONET_SESSION)?.into_iter().filter_map(node_from_peer).collect())
}

fn node_from_peer(peer: HyperNETPeer) -> Option<ONETNode> {
    let record: ONETNodeRecord = serde_json::from_str(&peer.player_name).ok()?;
    validate_record(&record).ok()?;
    Some(ONETNode { agent: peer.agent, record })
}

/// Sends one ONET frame to one agent as a remote signal (fire and forget; ONET replies are frames of their own).
#[hdk_extern]
pub fn onet_send(input: ONETSendInput) -> ExternResult<()> {
    if input.frame.is_empty() || input.frame.len() > MAX_FRAME_BYTES {
        return Err(invalid(&format!("ONET frame must be 1..={MAX_FRAME_BYTES} bytes")));
    }
    if input.to == agent_info()?.agent_initial_pubkey {
        return Err(invalid("an ONET frame cannot be sent to this agent itself"));
    }
    send_to(signal(ONET_FRAME_KIND, ONET_SESSION, "", Some(input.frame))?, vec![input.to])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record() -> ONETNodeRecord {
        ONETNodeRecord {
            node_id: "a".repeat(64),
            public_key: "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE".into(),
            advertised_address: "node.example:9000".into(),
            signature: "c2ln".into(),
        }
    }

    #[test]
    fn accepts_well_formed_record() {
        assert!(validate_record(&record()).is_ok());
    }

    #[test]
    fn rejects_node_id_that_is_not_a_sha256_hex() {
        let mut r = record();
        r.node_id = "MongoDBOASIS_node".into();
        assert!(validate_record(&r).is_err());
        r.node_id = "A".repeat(64);
        assert!(validate_record(&r).is_err());
    }

    #[test]
    fn rejects_unsigned_or_oversized_record() {
        let mut r = record();
        r.signature = String::new();
        assert!(validate_record(&r).is_err());
        let mut r = record();
        r.advertised_address = "x".repeat(MAX_NODE_RECORD_BYTES);
        assert!(validate_record(&r).is_err());
    }
}
