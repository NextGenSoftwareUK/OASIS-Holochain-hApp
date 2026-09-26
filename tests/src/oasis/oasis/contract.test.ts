import { assert, test } from 'vitest';
import { runScenario } from '@holochain/tryorama';
import type { Record } from '@holochain/client';

const app = { appBundleSource: { path: process.cwd() + '/../workdir/oasis.happ' } };

test('OASIS provider CRUD/index contract and DHT secret invariant', async () => {
  await runScenario(async scenario => {
    const [alice] = await scenario.addPlayersWithApps([app]);
    const cell = alice.cells[0];
    const avatar = { id: crypto.randomUUID(), username: 'edge-alice', email: 'edge@example.test', name: 'Edge Alice', is_active: true };

    const created = await cell.callZome<Record>({ zome_name: 'oasis', fn_name: 'create_entry_avatar', payload: avatar });
    assert.ok(created.signed_action.hashed.hash);
    const byId = await cell.callZome<Record>({ zome_name: 'oasis', fn_name: 'get_avatar_by_id', payload: avatar.id });
    assert.ok(byId.signed_action.hashed.hash);

    const updated = await cell.callZome<Record>({
      zome_name: 'oasis', fn_name: 'update_entry_avatar',
      payload: { original_action_hash: created.signed_action.hashed.hash, updated_entry: { ...avatar, name: 'Updated Edge Alice' } },
    });
    assert.ok(updated.signed_action.hashed.hash);

    await assert.rejects(() => cell.callZome({
      zome_name: 'oasis', fn_name: 'create_entry_avatar', payload: { ...avatar, id: crypto.randomUUID(), password: 'must-not-enter-dht' },
    }));
  });
});

test('HyperDrive mutation replay is idempotent', async () => {
  await runScenario(async scenario => {
    const [alice] = await scenario.addPlayersWithApps([app]);
    const cell = alice.cells[0];
    const mutation = { operation_id: crypto.randomUUID(), avatar_id: crypto.randomUUID(), entity_id: crypto.randomUUID(), entity_type: 'holon', kind: 0, version_id: crypto.randomUUID(), payload_json: '{"name":"offline"}' };
    const first = await cell.callZome<Record>({ zome_name: 'oasis', fn_name: 'apply_hyperdrive_mutation', payload: mutation });
    const replay = await cell.callZome<Record>({ zome_name: 'oasis', fn_name: 'apply_hyperdrive_mutation', payload: mutation });
    assert.deepEqual(replay.signed_action.hashed.hash, first.signed_action.hashed.hash);
  });
});
