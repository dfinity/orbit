<template>
  <div v-if="isListMode" class="d-flex flex-column ga-0 text-caption">
    <ProposalOperationListRow v-if="operation.input.resource">
      <template #name>{{ $t('terms.resource') }}</template>
      <template #content>
        {{
          $t(
            `access_policies.resources.${fromResourceToResourceEnum(operation.input.resource).toLowerCase()}`,
          )
        }}
      </template>
    </ProposalOperationListRow>
  </div>
  <VProgressCircular v-else-if="loading" />
  <AccessPolicyForm v-else :model-value="accessPolicy" mode="view" />
</template>

<script setup lang="ts">
import { computed, onBeforeMount, ref } from 'vue';
import { VProgressCircular } from 'vuetify/components';
import AccessPolicyForm from '~/components/access-policies/AccessPolicyForm.vue';
import logger from '~/core/logger.core';
import { AccessPolicy, EditAccessPolicyOperation, Proposal } from '~/generated/station/station.did';
import { fromResourceToResourceEnum } from '~/mappers/access-policies.mapper';
import { useWalletStore } from '~/stores/wallet.store';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    operation: EditAccessPolicyOperation;
    mode?: 'list' | 'detail';
  }>(),
  {
    mode: 'list',
  },
);

const isListMode = computed(() => props.mode === 'list');
const wallet = useWalletStore();
const accessPolicy = ref<Partial<AccessPolicy>>({});
const loading = ref(false);

const fetchDetails = async () => {
  try {
    if (loading.value || isListMode.value) {
      return;
    }

    loading.value = true;
    const { policy } = await wallet.service.getAccessPolicy({
      resource: props.operation.input.resource,
    });

    policy.allow.auth_scope = props.operation.input.auth_scope?.[0] ?? policy.allow.auth_scope;
    policy.allow.users = props.operation.input.users?.[0] ?? policy.allow.users;
    policy.allow.user_groups = props.operation.input.user_groups?.[0] ?? policy.allow.user_groups;

    accessPolicy.value = policy;
  } catch (e) {
    logger.error('Failed to fetch access policy details', e);
  } finally {
    loading.value = false;
  }
};

onBeforeMount(() => {
  fetchDetails();
});
</script>
