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
import AccessPolicyForm from '~/components/access-policies/AccessPolicyForm.vue';
import { AccessPolicy, EditAccessPolicyOperation, Proposal } from '~/generated/wallet/wallet.did';
import { fromResourceToResourceEnum } from '~/mappers/access-policies.mapper';
import ProposalOperationListRow from '../ProposalOperationListRow.vue';
import { variantIs } from '~/utils/helper.utils';
import { VProgressCircular } from 'vuetify/components';
import logger from '~/core/logger.core';
import { useWalletStore } from '~/stores/wallet.store';

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
    const current = await wallet.service.getAccessPolicy({
      resource: props.operation.input.resource,
    });
    accessPolicy.value = current.policy;
    const currentAuthentication = accessPolicy.value.allow?.authentication ?? [];
    const currentUserGroups = accessPolicy.value.allow?.user_groups ?? [];
    const currentUsers = accessPolicy.value.allow?.users ?? [];
    if (variantIs(props.operation.input.access, 'Allow')) {
      accessPolicy.value.allow = {
        authentication: variantIs(props.operation.input.access.Allow, 'authentication')
          ? props.operation.input.access.Allow.authentication
          : currentAuthentication,
        user_groups: variantIs(props.operation.input.access.Allow, 'user_groups')
          ? props.operation.input.access.Allow.user_groups
          : currentUserGroups,
        users: variantIs(props.operation.input.access.Allow, 'users')
          ? props.operation.input.access.Allow.users
          : currentUsers,
      };
    } else if (variantIs(props.operation.input.access, 'Deny')) {
      accessPolicy.value.allow = {
        user_groups: variantIs(props.operation.input.access.Deny, 'UserGroups')
          ? []
          : currentUserGroups,
        users: variantIs(props.operation.input.access.Deny, 'Users') ? [] : currentUsers,
        authentication:
          variantIs(props.operation.input.access.Deny, 'Any') ||
          variantIs(props.operation.input.access.Deny, 'Authenticated')
            ? []
            : currentAuthentication,
      };
    }
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
