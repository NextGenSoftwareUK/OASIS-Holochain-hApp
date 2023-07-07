import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleAvatar(cell: CallableCell, partialAvatar = {}) {
    return {
        ...{
	  first_name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  last_name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  dob: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  created_date: 1674053334548000,
	  created_by: (await fakeAgentPubKey()),
	  modified_by: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialAvatar
    };
}

export async function createAvatar(cell: CallableCell, avatar = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "oasis",
      fn_name: "create_avatar",
      payload: avatar || await sampleAvatar(cell),
    });
}

