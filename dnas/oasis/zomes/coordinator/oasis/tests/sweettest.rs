use std::path::PathBuf;
use holochain::sweettest::{SweetConductor, SweetDnaFile};
use hdk::prelude::*;
use oasis_integrity::{Avatar, HyperDriveMutation};

fn avatar(id: &str) -> Avatar {
    Avatar {
        id: id.into(),
        username: "edge-alice".into(),
        email: "edge@example.test".into(),
        fields: serde_json::Map::from_iter([(String::from("name"), serde_json::Value::String("Edge Alice".into()))]),
    }
}

fn dna_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/oasis.dna")
}

#[tokio::test(flavor = "multi_thread")]
async fn provider_contract_runs_in_holochain_07_conductor() {
    let dna = SweetDnaFile::from_bundle(&dna_path()).await.unwrap();
    let mut conductor = SweetConductor::standard().await;
    let app = conductor.setup_app("oasis", [&dna]).await.unwrap();
    let cell = &app.into_cells()[0];
    let zome = cell.zome("oasis");

    let created: Record = conductor.call(&zome, "create_entry_avatar", avatar("avatar-1")).await;
    let loaded: Option<Record> = conductor.call(&zome, "get_avatar_by_id", "avatar-1".to_string()).await;
    assert!(loaded.is_some());

    let replayable = HyperDriveMutation {
        operation_id: "operation-1".into(), avatar_id: "avatar-1".into(), entity_id: "holon-1".into(),
        entity_type: "holon".into(), kind: 0, version_id: "version-1".into(), payload_json: Some("{}".into()),
    };
    let first: Record = conductor.call(&zome, "apply_hyperdrive_mutation", replayable.clone()).await;
    let replay: Record = conductor.call(&zome, "apply_hyperdrive_mutation", replayable).await;
    assert_eq!(first.action_address(), replay.action_address());
    assert_ne!(created.action_address(), first.action_address());
}

#[tokio::test(flavor = "multi_thread")]
async fn conductor_rejects_auth_secrets_in_public_avatar() {
    let dna = SweetDnaFile::from_bundle(&dna_path()).await.unwrap();
    let mut conductor = SweetConductor::standard().await;
    let app = conductor.setup_app("oasis", [&dna]).await.unwrap();
    let cell = &app.into_cells()[0];
    let mut unsafe_avatar = avatar("avatar-secret");
    unsafe_avatar.fields.insert("password".into(), serde_json::Value::String("never".into()));
    let result = conductor.call_fallible::<_, Record>(&cell.zome("oasis"), "create_entry_avatar", unsafe_avatar).await;
    assert!(result.is_err());
}

fn onet_record(seed: char, address: &str) -> oasis::onet::ONETNodeRecord {
    oasis::onet::ONETNodeRecord {
        node_id: seed.to_string().repeat(64),
        public_key: "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE".into(),
        advertised_address: address.into(),
        signature: "c2lnbmF0dXJl".into(),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn onet_nodes_register_discover_and_send_frames() {
    let dna = SweetDnaFile::from_bundle(&dna_path()).await.unwrap();
    let mut conductor = SweetConductor::standard().await;
    let apps = conductor.setup_apps("oasis", 2, [&dna]).await.unwrap();
    let ((alice,), (bob,)) = apps.into_tuples();

    let _: Vec<oasis::onet::ONETNode> =
        conductor.call(&alice.zome("oasis"), "onet_register_node", onet_record('a', "alice.example:9000")).await;
    holochain::sweettest::await_consistency([&alice]).await.unwrap();
    let registered: Vec<oasis::onet::ONETNode> =
        conductor.call(&bob.zome("oasis"), "onet_register_node", onet_record('b', "bob.example:9000")).await;
    assert_eq!(registered.len(), 2, "registration returns every ONET node, including this one");

    // Re-registering replaces the agent's record instead of adding a second one.
    let _: Vec<oasis::onet::ONETNode> =
        conductor.call(&alice.zome("oasis"), "onet_register_node", onet_record('a', "alice.example:9001")).await;
    holochain::sweettest::await_consistency([&alice, &bob]).await.unwrap();
    let nodes: Vec<oasis::onet::ONETNode> = conductor.call(&bob.zome("oasis"), "onet_nodes", ()).await;
    assert_eq!(nodes.len(), 2);
    let alice_node = nodes.iter().find(|n| n.agent == *alice.agent_pubkey()).unwrap();
    assert_eq!(alice_node.record.advertised_address, "alice.example:9001");

    let sent: Result<(), _> = conductor.call_fallible(&alice.zome("oasis"), "onet_send", oasis::onet::ONETSendInput {
        to: bob.agent_pubkey().clone(),
        frame: serde_bytes::ByteBuf::from(vec![1u8, 2, 3]),
    }).await;
    assert!(sent.is_ok(), "{sent:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn onet_rejects_malformed_records_and_self_frames() {
    let dna = SweetDnaFile::from_bundle(&dna_path()).await.unwrap();
    let mut conductor = SweetConductor::standard().await;
    let app = conductor.setup_app("oasis", [&dna]).await.unwrap();
    let cell = &app.into_cells()[0];

    let mut bridge = onet_record('a', "x:1");
    bridge.node_id = "MongoDBOASIS_node".into();
    let rejected = conductor.call_fallible::<_, Vec<oasis::onet::ONETNode>>(&cell.zome("oasis"), "onet_register_node", bridge).await;
    assert!(rejected.is_err());

    let to_self = conductor.call_fallible::<_, ()>(&cell.zome("oasis"), "onet_send", oasis::onet::ONETSendInput {
        to: cell.agent_pubkey().clone(),
        frame: serde_bytes::ByteBuf::from(vec![1u8]),
    }).await;
    assert!(to_self.is_err());
}
