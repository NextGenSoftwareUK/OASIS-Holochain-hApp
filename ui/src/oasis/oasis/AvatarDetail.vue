<template>
  <div v-if="!loading">
    <div v-if="editing" style="display: flex; flex: 1;">
      <EditAvatar
        :original-avatar-hash="avatarHash"
        :current-record="record!"
        @avatar-updated="editing = false; fetchAvatar();"
        @edit-canceled="editing = false"
      ></EditAvatar>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
      
        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteAvatar()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>First Name: </strong></span>
 	<span style="white-space: pre-line">{{  avatar?.first_name }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Last Name: </strong></span>
 	<span style="white-space: pre-line">{{  avatar?.last_name }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Dob: </strong></span>
 	<span style="white-space: pre-line">{{  avatar?.dob }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Created Date: </strong></span>
 	<span style="white-space: pre-line">{{  new Date(avatar?.created_date / 1000).toLocaleString() }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Modified By: </strong></span>
 	<span style="white-space: pre-line">{{  avatar?.modified_by }} </span>
      </div>

    </div>
    
    <span v-else>The requested avatar was not found.</span>
  </div>

  <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <mwc-snackbar ref="delete-error" leading>
  </mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Avatar } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditAvatar from './EditAvatar.vue';

export default defineComponent({
  components: {
    EditAvatar
  },
  props: {
    avatarHash: {
      type: Object,
      required: true
    }
  },
  data(): { record: Record | undefined; loading: boolean; editing: boolean; } {
    return {
      record: undefined,
      loading: true,
      editing: false,
    }
  },
  computed: {
    avatar() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Avatar;
    }
  },
  async mounted() {
    if (this.avatarHash === undefined) {
      throw new Error(`The avatarHash input is required for the AvatarDetail element`);
    }

    await this.fetchAvatar();
  },
  methods: {
    async fetchAvatar() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'oasis',
        zome_name: 'oasis',
        fn_name: 'get_avatar',
        payload: this.avatarHash,
      });

      this.loading = false;
    },
    async deleteAvatar() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'oasis',
          zome_name: 'oasis',
          fn_name: 'delete_avatar',
          payload: this.avatarHash,
        });
        this.$emit('avatar-deleted', this.avatarHash);
        this.fetchAvatar();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the avatar: ${e.data.data}`;
        errorSnackbar.show();
      }
    }
  },
  emits: ['avatar-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>
