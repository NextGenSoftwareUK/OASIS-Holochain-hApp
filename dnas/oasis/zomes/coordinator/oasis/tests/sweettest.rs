use std::path::PathBuf;
use holochain::sweettest::{SweetConductor, SweetDnaFile};
use hdk::prelude::*;
use oasis_integrity::{Avatar, AvatarDetail, Holon, HyperDriveMutation};

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
    let replay: Record = conductor.call(&zome, "apply_hyperdrive_mutation", replayable.clone()).await;
    assert_eq!(first.action_address(), replay.action_address());
    assert_ne!(created.action_address(), first.action_address());

    let mut conflicting = replayable.clone();
    conflicting.version_id = "version-conflict".into();
    conflicting.payload_json = Some("{\"changed\":true}".into());
    let rejected = conductor.call_fallible::<_, Record>(
        &zome, "apply_hyperdrive_mutation", conflicting).await;
    let error = rejected.expect_err("a different mutation cannot reuse the operation ID");
    assert!(format!("{error:?}").contains("already bound to a different mutation"));

    let stored: Option<Record> = conductor.call(
        &zome, "get_hyperdrive_mutation", replayable.operation_id.clone()).await;
    let stored = stored.expect("original mutation must remain indexed");
    assert_eq!(stored.action_address(), first.action_address());
    let payload: HyperDriveMutation = stored.entry().to_app_option().unwrap().unwrap();
    assert!(payload == replayable, "original mutation content changed");

    let latest: Option<Record> = conductor.call(&zome, "get_latest_hyperdrive_entity",
        serde_json::json!({
            "avatar_id": &replayable.avatar_id,
            "entity_type": &replayable.entity_type,
            "entity_id": &replayable.entity_id,
        })).await;
    assert_eq!(latest.unwrap().action_address(), first.action_address());
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

#[tokio::test(flavor = "multi_thread")]
async fn identity_deletion_removes_prior_revisions_and_avatar_detail() {
    let dna = SweetDnaFile::from_bundle(&dna_path()).await.unwrap();
    let mut conductor = SweetConductor::standard().await;
    let app = conductor.setup_app("oasis", [&dna]).await.unwrap();
    let cell = &app.into_cells()[0];
    let zome = cell.zome("oasis");

    #[derive(Debug, Serialize)]
    struct Update<T> {
        original_action_hash: ActionHash,
        updated_entry: T,
    }

    let original: Record = conductor.call(&zome, "create_entry_avatar", avatar("avatar-delete")).await;
    let mut updated_avatar = avatar("avatar-delete");
    updated_avatar.username = "renamed-avatar".into();
    let updated: Record = conductor.call(&zome, "update_entry_avatar", Update {
        original_action_hash: original.action_address().clone(),
        updated_entry: updated_avatar,
    }).await;
    let detail = AvatarDetail {
        id: "avatar-delete".into(),
        username: "renamed-avatar".into(),
        email: "edge@example.test".into(),
        fields: Default::default(),
    };
    let _: Record = conductor.call(&zome, "create_entry_avatar_detail", detail).await;
    let _: Record = conductor.call(&zome, "create_entry_avatar", avatar("avatar-keep")).await;
    let deleted: Record = conductor.call(&zome, "delete_avatar_by_id", "avatar-delete").await;
    assert_eq!(deleted.action_address(), updated.action_address());
    let avatar: Option<Record> = conductor.call(&zome, "get_avatar_by_id", "avatar-delete").await;
    let detail: Option<Record> = conductor.call(&zome, "get_avatar_detail_by_id", "avatar-delete").await;
    let kept: Option<Record> = conductor.call(&zome, "get_avatar_by_id", "avatar-keep").await;
    assert!(avatar.is_none());
    assert!(detail.is_none());
    assert!(kept.is_some());

    let holon: Holon = serde_json::from_value(serde_json::json!({"id": "holon-delete"})).unwrap();
    let original: Record = conductor.call(&zome, "create_entry_holon", holon).await;
    let updated_holon: Holon = serde_json::from_value(serde_json::json!({
        "id": "holon-delete", "name": "updated"
    })).unwrap();
    let updated: Record = conductor.call(&zome, "update_entry_holon", Update {
        original_action_hash: original.action_address().clone(),
        updated_entry: updated_holon,
    }).await;
    let kept_holon: Holon = serde_json::from_value(serde_json::json!({"id": "holon-keep"})).unwrap();
    let kept: Record = conductor.call(&zome, "create_entry_holon", kept_holon).await;
    let before: Vec<Record> = conductor.call(&zome, "get_all_holons", ()).await;
    assert_eq!(
        before.iter().map(Record::action_address).collect::<Vec<_>>(),
        vec![original.action_address(), updated.action_address(), kept.action_address()]
    );
    let deleted: Record = conductor.call(&zome, "delete_holon_by_id", "holon-delete").await;
    assert_eq!(deleted.action_address(), updated.action_address());
    let holon: Option<Record> = conductor.call(&zome, "get_holon_by_id", "holon-delete").await;
    let all: Vec<Record> = conductor.call(&zome, "get_all_holons", ()).await;
    assert!(holon.is_none());
    assert_eq!(all.iter().map(Record::action_address).collect::<Vec<_>>(), vec![kept.action_address()]);
}
