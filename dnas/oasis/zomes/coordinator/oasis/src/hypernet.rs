//! HoloNET HyperNET: real-time peer-to-peer sessions (e.g. multiplayer games).
//!
//! Players join a named session; membership is a link from the session's anchor path to each player's agent key (tag =
//! player name). Messages go straight from agent to agent as Holochain *remote signals*: nothing is written to the
//! source chain or the DHT, which is what keeps them fast. Each receiving cell hands the message to its client
//! (HoloNET) with `emit_signal`. Coordinator-only, so the DNA hash (and existing data) is unchanged.

use hdk::prelude::*;
use oasis_integrity::*;

/// What every HyperNET signal looks like when it reaches a client (kind = "joined" | "left" | "message").
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HyperNETSignal {
    pub hypernet: bool,
    pub kind: String,
    pub session: String,
    pub from: AgentPubKey,
    pub player_name: String,
    pub payload: Option<serde_bytes::ByteBuf>,
    pub sent_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HyperNETPeer {
    pub agent: AgentPubKey,
    pub player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyperNETJoinInput {
    pub session: String,
    pub player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyperNETSendInput {
    pub session: String,
    pub payload: serde_bytes::ByteBuf,
    /// Only these players (None = everyone else in the session).
    #[serde(default)]
    pub to: Option<Vec<AgentPubKey>>,
}

fn session_base(session: &str) -> ExternResult<EntryHash> {
    Path::from(format!("hypernet.session.{}", session.replace('.', "_dot_").replace('/', "_slash_"))).path_entry_hash()
}

fn session_links(session: &str) -> ExternResult<Vec<Link>> {
    // Reuses an existing link type so the integrity zome (and the DNA hash) doesn't change; the base path keeps
    // these links apart from everything else.
    get_links(LinkQuery::try_new(session_base(session)?, LinkTypes::AllAvatars)?, GetStrategy::Network)
}

fn now_micros() -> ExternResult<i64> {
    Ok(sys_time()?.as_micros())
}

fn peers(session: &str) -> ExternResult<Vec<HyperNETPeer>> {
    let mut peers: Vec<HyperNETPeer> = Vec::new();
    for link in session_links(session)? {
        if let Ok(agent) = AgentPubKey::try_from(link.target.clone()) {
            if peers.iter().any(|p| p.agent == agent) {
                continue;
            }
            let player_name = String::from_utf8(link.tag.into_inner()).unwrap_or_default();
            peers.push(HyperNETPeer { agent, player_name });
        }
    }
    Ok(peers)
}

fn signal(kind: &str, session: &str, player_name: &str, payload: Option<serde_bytes::ByteBuf>) -> ExternResult<HyperNETSignal> {
    Ok(HyperNETSignal {
        hypernet: true,
        kind: kind.into(),
        session: session.into(),
        from: agent_info()?.agent_initial_pubkey,
        player_name: player_name.into(),
        payload,
        sent_at: now_micros()?,
    })
}

fn others(session: &str) -> ExternResult<Vec<AgentPubKey>> {
    let me = agent_info()?.agent_initial_pubkey;
    Ok(peers(session)?.into_iter().map(|p| p.agent).filter(|a| *a != me).collect())
}

/// Join (or re-join) a session; returns everyone in it, including you. The others get a "joined" signal.
#[hdk_extern]
pub fn hypernet_join(input: HyperNETJoinInput) -> ExternResult<Vec<HyperNETPeer>> {
    let me = agent_info()?.agent_initial_pubkey;
    let base = session_base(&input.session)?;

    // One membership link per player: drop an old one (e.g. a new player name) first.
    for link in session_links(&input.session)? {
        if AgentPubKey::try_from(link.target.clone()).map(|a| a == me).unwrap_or(false) {
            delete_link(link.create_link_hash, GetOptions::default())?;
        }
    }
    create_link(base, me, LinkTypes::AllAvatars, LinkTag::new(input.player_name.as_bytes().to_vec()))?;

    let targets = others(&input.session)?;
    if !targets.is_empty() {
        send_to(signal("joined", &input.session, &input.player_name, None)?, targets)?;
    }
    peers(&input.session)
}

/// Leave a session. The others get a "left" signal.
#[hdk_extern]
pub fn hypernet_leave(session: String) -> ExternResult<()> {
    let me = agent_info()?.agent_initial_pubkey;
    let mut player_name = String::new();
    for link in session_links(&session)? {
        if AgentPubKey::try_from(link.target.clone()).map(|a| a == me).unwrap_or(false) {
            player_name = String::from_utf8(link.tag.clone().into_inner()).unwrap_or_default();
            delete_link(link.create_link_hash, GetOptions::default())?;
        }
    }
    let targets = others(&session)?;
    if !targets.is_empty() {
        send_to(signal("left", &session, &player_name, None)?, targets)?;
    }
    Ok(())
}

/// Everyone currently in a session.
#[hdk_extern]
pub fn hypernet_peers(session: String) -> ExternResult<Vec<HyperNETPeer>> {
    peers(&session)
}

/// Send bytes to the other players (or just `to`). Fire and forget: delivered as remote signals, not stored.
#[hdk_extern]
pub fn hypernet_send(input: HyperNETSendInput) -> ExternResult<u32> {
    let me = agent_info()?.agent_initial_pubkey;
    let targets: Vec<AgentPubKey> = match input.to {
        Some(to) => to.into_iter().filter(|a| *a != me).collect(),
        None => others(&input.session)?,
    };
    let count = targets.len() as u32;
    if count > 0 {
        send_to(signal("message", &input.session, "", Some(input.payload))?, targets)?;
    }
    Ok(count)
}

/// A remote signal arrived from a peer: hand it to this cell's client (HoloNET raises it as an app signal).
#[hdk_extern]
/// The conductor hands over the signal exactly as send_remote_signal encoded it, so take the struct itself
/// (an ExternIO parameter fails to deserialize every real remote signal).
pub fn recv_remote_signal(signal: HyperNETSignal) -> ExternResult<()> {
    emit_signal(signal)
}

/// Lets any peer deliver remote signals to this cell (Holochain requires a grant for recv_remote_signal).
/// Holochain 0.7.0 shape: { tag, access, functions } (0.7.1 changed it).
pub fn grant_remote_signals() -> ExternResult<()> {
    let mut functions = HashSet::new();
    functions.insert((zome_info()?.name, FunctionName("recv_remote_signal".into())));
    create_cap_grant(CapGrantEntry {
        tag: "hypernet-remote-signals".into(),
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
    })?;
    Ok(())
}

/// One send_remote_signal per player: given several agents on the same conductor, Holochain delivers a multi-target
/// signal to only one of them (seen on 0.7.1 with three local agents), so never rely on a multi-target send.
fn send_to(signal: HyperNETSignal, targets: Vec<AgentPubKey>) -> ExternResult<()> {
    for target in targets {
        send_remote_signal(signal.clone(), vec![target])?;
    }
    Ok(())
}
