<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Avatar</span>
  
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

  
    <mwc-button 
      raised
      label="Create Avatar"
      :disabled="!isAvatarValid"
      @click="createAvatar"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Avatar } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';

import '@material/mwc-textfield';
export default defineComponent({
  data(): {
    firstName: string;
    lastName: string;
    dob: string;
    createdDate: number;
    modifiedBy: string;
  } {
    return { 
      firstName: '',
      lastName: '',
      dob: '',
      createdDate: Date.now(),
      modifiedBy: '',
    }
  },
  props: {
    createdBy: {
      type: null,
      required: true
    },
  },
  computed: {
    isAvatarValid() {
    return true && this.firstName !== '' && this.lastName !== '' && this.dob !== '' && true && this.modifiedBy !== '';
    },
  },
  mounted() {
    if (this.createdBy === undefined) {
      throw new Error(`The createdBy input is required for the CreateAvatar element`);
    }
  },
  methods: {
    async createAvatar() {
      const avatar: Avatar = { 
        first_name: this.firstName!,

        last_name: this.lastName!,

        dob: this.dob!,

        created_date: this.createdDate!,

        created_by: this.createdBy!,

        modified_by: this.modifiedBy!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'oasis',
          zome_name: 'oasis',
          fn_name: 'create_avatar',
          payload: avatar,
        });
        this.$emit('avatar-created', record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the avatar: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['avatar-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
