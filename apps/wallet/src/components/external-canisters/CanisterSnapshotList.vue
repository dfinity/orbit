<template>
  <section class="d-flex flex-column ga-2">
    <DataLoader :load="loadSnapshots" @loading="loading = $event">
      <template #error="{ errorMsg, errorDetails }">
        <ErrorCard :error="errorMsg" :error-details="errorDetails" />
      </template>
      <LoadingMessage v-if="loading" class="ml-4" />
      <div v-else>
        <VRow v-if="snapshots.length" data-test-id="snapshots-list">
          <VCol cols="12">
            <VTable hover class="elevation-2 rounded w-100">
              <thead>
                <tr>
                  <th>{{ $t('external_canisters.snapshots.id') }}</th>
                  <th>{{ $t('external_canisters.snapshots.size_mb') }}</th>
                  <th>{{ $t('external_canisters.snapshots.created_at') }}</th>
                  <th v-if="!readonly">&nbsp;</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="snapshot in snapshots" :key="snapshot.snapshotId">
                  <td>{{ snapshot.snapshotId }}</td>
                  <td>{{ (snapshot.totalSize / 1_000_000).toFixed(2) }}</td>
                  <td>
                    {{
                      `${new Date(snapshot.takenAtTimestamp).toLocaleDateString()} ${new Date(snapshot.takenAtTimestamp).toLocaleTimeString()}`
                    }}
                  </td>
                  <td v-if="!readonly" class="text-right">
                    <CanisterSnapshotRestoreDialog
                      v-model:open="dialogs.restoreSnapshot"
                      :canister-id="props.canisterId"
                      :snapshot="snapshot"
                      :title="$t('external_canisters.snapshots.restore_snapshot_title')"
                    />

                    <CanisterSnapshotRemoveDialog
                      v-model:open="dialogs.removeSnapshot"
                      :canister-id="props.canisterId"
                      :snapshot="snapshot"
                      :title="$t('external_canisters.snapshots.remove_snapshot_title')"
                    />

                    <VMenu>
                      <template #activator="{ props: activatorProps }">
                        <VBtn
                          :icon="mdiDotsVertical"
                          density="comfortable"
                          v-bind="activatorProps"
                        />
                      </template>
                      <VList density="compact">
                        <VListItem
                          :key="`restore-${snapshot.snapshotId}`"
                          :prepend-icon="mdiRestore"
                          @click="dialogs.restoreSnapshot = true"
                        >
                          <VListItemTitle>{{ $t('terms.restore') }}</VListItemTitle>
                        </VListItem>
                        <VListItem
                          :key="`remove-${snapshot.snapshotId}`"
                          :prepend-icon="mdiDelete"
                          @click="dialogs.removeSnapshot = true"
                        >
                          <VListItemTitle>{{ $t('terms.remove') }}</VListItemTitle>
                        </VListItem>
                      </VList>
                    </VMenu>
                  </td>
                </tr>
              </tbody>
            </VTable>
          </VCol>
        </VRow>
        <p v-else data-test-id="empty-snapshots-list" class="px-4">
          {{ $t('external_canisters.snapshots.no_snapshots') }}
        </p>
        <footer v-if="hasInstalledWasm" class="d-flex flex-md-row flex-column ga-2 mt-4">
          <CanisterSnapshotCreateDialog
            v-model:open="dialogs.createSnapshot"
            :canister-id="props.canisterId"
            :title="$t('external_canisters.snapshots.create_snapshot')"
          />
          <VBtn
            v-if="!readonly"
            :disabled="snapshots.length >= MAX_SNAPSHOTS"
            size="small"
            variant="outlined"
            block
            @click="dialogs.createSnapshot = true"
          >
            {{ $t('external_canisters.snapshots.create_snapshot') }}
          </VBtn>
        </footer>
      </div>
    </DataLoader>
  </section>
</template>
<script setup lang="ts">
import { Principal } from '@dfinity/principal';
import { computed, ref, toRefs, watch } from 'vue';
import { VBtn } from 'vuetify/components';
import { useStationStore } from '~/stores/station.store';
import DataLoader from '../DataLoader.vue';
import LoadingMessage from '../LoadingMessage.vue';
import ErrorCard from '../ui/ErrorCard.vue';
import CanisterSnapshotCreateDialog from './CanisterSnapshotCreateDialog.vue';
import { mdiDelete, mdiDotsVertical, mdiRestore } from '@mdi/js';
import { debounce } from '~/utils/helper.utils';
import { CanisterSnapshot } from './external-canisters.types';
import CanisterSnapshotRestoreDialog from './CanisterSnapshotRestoreDialog.vue';
import CanisterSnapshotRemoveDialog from './CanisterSnapshotRemoveDialog.vue';

const props = withDefaults(
  defineProps<{
    canisterId: Principal;
    hasInstalledWasm?: boolean;
    readonly?: boolean;
  }>(),
  {
    hasInstalledWasm: false,
    readonly: false,
  },
);

// todo: once the ICP api enables more than one snapshot, this restriction can be removed
const MAX_SNAPSHOTS = 1;

const { readonly, hasInstalledWasm } = toRefs(props);

const dialogs = ref({
  createSnapshot: false,
  restoreSnapshot: false,
  removeSnapshot: false,
});

const loading = ref<boolean>(false);
const snapshots = ref<CanisterSnapshot[]>([]);
const station = useStationStore();

const emit = defineEmits<{
  (event: 'editing', payload: boolean): void;
}>();

const loadSnapshots = async (): Promise<void> => {
  const response = await station.service.getExernalCanisterSnapshots(props.canisterId);

  snapshots.value = response.map(snapshot => ({
    snapshotId: snapshot.snapshot_id,
    totalSize: Number(snapshot.total_size),
    takenAtTimestamp: snapshot.taken_at_timestamp,
  }));
};

const debouncedLoadSnapshots = debounce(() => loadSnapshots(), 2_000);

const isEditing = computed(
  () =>
    dialogs.value.createSnapshot || dialogs.value.restoreSnapshot || dialogs.value.removeSnapshot,
);

watch(isEditing, value => {
  emit('editing', value);

  if (!value) {
    debouncedLoadSnapshots();
  }
});
</script>
