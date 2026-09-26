import { assert, test } from "vitest";
import { runScenario } from "@holochain/tryorama";
import { Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";

const appSource = () => ({
  appBundleSource: { path: process.cwd() + "/../workdir/oasis.happ" },
});

function mutation(operationId: string, versionId: string, payload: string) {
  return {
    operation_id: operationId,
    avatar_id: "10000000-0000-0000-0000-000000000001",
    entity_id: "20000000-0000-0000-0000-000000000001",
    entity_type: "holon",
    kind: 0,
    version_id: versionId,
    payload_json: payload,
  };
}

test("HyperDrive operation replay returns the original record", async () => {
  await runScenario(async scenario => {
    const [alice] = await scenario.addPlayersWithApps([appSource()]);
    const value = mutation("30000000-0000-0000-0000-000000000001",
      "40000000-0000-0000-0000-000000000001", '{"value":1}');

    const first: Record = await alice.cells[0].callZome({
      zome_name: "oasis", fn_name: "apply_hyperdrive_mutation", payload: value,
    });
    const replay: Record = await alice.cells[0].callZome({
      zome_name: "oasis", fn_name: "apply_hyperdrive_mutation", payload: value,
    });

    assert.deepEqual(replay.signed_action.hashed.hash, first.signed_action.hashed.hash);
    assert.deepEqual(decode((replay.entry as any).Present.entry), value);
  });
});

test("HyperDrive entity index returns the latest appended version", async () => {
  await runScenario(async scenario => {
    const [alice] = await scenario.addPlayersWithApps([appSource()]);
    const first = mutation("30000000-0000-0000-0000-000000000011",
      "40000000-0000-0000-0000-000000000011", '{"value":1}');
    const second = mutation("30000000-0000-0000-0000-000000000012",
      "40000000-0000-0000-0000-000000000012", '{"value":2}');
    await alice.cells[0].callZome({
      zome_name: "oasis", fn_name: "apply_hyperdrive_mutation", payload: first,
    });
    await alice.cells[0].callZome({
      zome_name: "oasis", fn_name: "apply_hyperdrive_mutation", payload: second,
    });

    const latest: Record = await alice.cells[0].callZome({
      zome_name: "oasis", fn_name: "get_latest_hyperdrive_entity",
      payload: { avatar_id: second.avatar_id, entity_type: second.entity_type,
        entity_id: second.entity_id },
    });
    assert.deepEqual(decode((latest.entry as any).Present.entry), second);
  });
});
