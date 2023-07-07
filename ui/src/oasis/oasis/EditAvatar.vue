<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Avatar</span>
      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="First Name" :value="firstName" @input="firstName = $event.target.value" required></mwc-textfield>
      </div>

      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Last Name" :value="lastName" @input="lastName = $event.target.value" required></mwc-textfield>
      </div>

      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Dob" :value="dob" @input="dob = $event.target.value" required></mwc-textfield>
      </div>

      <div style="margin-bottom: 16px">
      <vaadin-date-time-picker label="Created Date" :value="new Date(createdDate / 1000).toISOString()" @change="createdDate = new Date($event.target.value).valueOf() * 1000" required></vaadin-date-time-picker>
      </div>

      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Modified By" :value="modifiedBy" @input="modifiedBy = $event.target.value" required></mwc-textfield>
      </div>



    <div style="display: flex; flex-direction: row">
      <mwc-button
        outlined
        label="Cancel"
        @click="$emit('edit-canceled')"
        style="flex: 1; margin-right: 16px;"
      ></mwc-button>
      <mwc-button 
        raised
        label="Save"
        :disabled="!isAvatarValid"
        @click="updateAvatar"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Avatar } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textfield';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
export default defineComponent({
  data(): {
    firstName: string;
    lastName: string;
    dob: string;
    createdDate: number;
    modifiedBy: string;
  } {
    const currentAvatar = decode((this.currentRecord.entry as any).Present.entry) as Avatar;
    return { 
      firstName: currentAvatar.firstName,
      lastName: currentAvatar.lastName,
      dob: currentAvatar.dob,
      createdDate: currentAvatar.createdDate,
      modifiedBy: currentAvatar.modifiedBy,
    }
  },
  props: {
    originalAvatarHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentAvatar() {
      return decode((this.currentRecord.entry as any).Present.entry) as Avatar;
    },
    isAvatarValid() {
      return true && this.firstName !== '' && this.lastName !== '' && this.dob !== '' && true && this.modifiedBy !== '';
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditAvatar element`);
    }
    if (this.originalAvatarHash === undefined) {
      throw new Error(`The originalAvatarHash input is required for the EditAvatar element`);
    }
  },
  methods: {
    async updateAvatar() {

      const avatar: Avatar = { 
        first_name: this.firstName,
        last_name: this.lastName,
        dob: this.dob,
        created_date: this.createdDate,
        created_by: this.currentAvatar.created_by,
        modified_by: this.modifiedBy,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'oasis',
          zome_name: 'oasis',
          fn_name: 'update_avatar',
          payload: {
            original_avatar_hash: this.originalAvatarHash,
            previous_avatar_hash: this.currentRecord.signed_action.hashed.hash,
            updated_avatar: avatar
          }
        });
        this.$emit('avatar-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the avatar: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['avatar-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
