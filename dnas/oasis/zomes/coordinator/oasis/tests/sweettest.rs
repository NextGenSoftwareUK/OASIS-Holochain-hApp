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
