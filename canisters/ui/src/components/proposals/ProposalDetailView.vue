<template v-if="proposalReviewComponent">
  <VCard :loading="props.loading">
    <VToolbar color="transparent">
      <VToolbarTitle>
        <span class="text-body-2 font-weight-light">
          {{ $t(`proposals.types.${proposalType}.request_title`) }}
        </span>
        <br />
        <span v-if="props.proposal.title">
          {{ props.proposal.title }}
          <VTooltip
            v-model="titleTooltip"
            location="bottom"
            :open-on-hover="false"
            :open-on-click="true"
            @click:outside="titleTooltip = false"
          >
            <template #activator="{ props: infoProps }">
              <VBtn :icon="mdiInformationOutline" size="x-small" v-bind="infoProps" />
            </template>
            {{ $t('proposals.title_info_message') }}
          </VTooltip>
        </span>
      </VToolbarTitle>
      <VSpacer />
      <slot name="top-actions"></slot>
    </VToolbar>
    <VCardText class="px-4 pt-2">
      <VContainer class="px-0">
        <VRow v-if="props.proposal.summary?.[0]">
          <VCol cols="12" class="text-h6 font-weight-bold">
            <VTextarea
              :model-value="props.proposal.summary[0]"
              :label="$t('terms.summary')"
              variant="plain"
              readonly
              hide-details
              rows="1"
              auto-grow
              class="my-2"
            />
          </VCol>
        </VRow>
        <VRow>
          <VCol cols="12" class="text-body-1 font-weight-bold py-0">
            {{ $t('terms.requested') }}
          </VCol>
        </VRow>
        <VRow>
          <VCol cols="12">
            <component
              :is="detailView?.component"
              v-if="detailView"
              :proposal="proposal"
              :operation="detailView.operation"
            />
          </VCol>
        </VRow>
      </VContainer>
    </VCardText>
    <VCardActions class="pa-4 d-flex flex-column-reverse flex-md-row ga-2">
      <ProposalMetadata
        :proposal="props.proposal"
        :details="props.details"
        class="flex-grow-0 mt-md-0"
        :class="{ 'mt-8': props.details.can_vote }"
      />
      <div class="d-flex flex-column flex-md-row ga-2 justify-end flex-grow-1 w-100 w-md-auto">
        <template v-if="props.details.can_vote">
          <VBtn variant="outlined" :disabled="props.loading" @click="$emit('approve')">
            {{ $t('terms.approve') }}
          </VBtn>
          <VBtn variant="outlined" :disabled="props.loading" class="ma-0" @click="$emit('reject')">
            {{ $t('terms.reject') }}
          </VBtn>
        </template>
        <template v-else>
          <ProposalStatusChip :status="proposal.status" />
          <VDivider class="d-md-none mx-2" />
        </template>
      </div>
    </VCardActions>
  </VCard>
</template>

<script setup lang="ts">
import { mdiInformationOutline } from '@mdi/js';
import type { Component } from 'vue';
import { computed, ref } from 'vue';
import { Proposal, ProposalOperation } from '~/generated/wallet/wallet.did';
import { ProposalDetails } from '~/types/wallet.types';
import { KeysOfUnion } from '~/utils/helper.utils';
import ProposalMetadata from './ProposalMetadata.vue';
import ProposalStatusChip from './ProposalStatusChip.vue';
import ReviewUnknownOperation from './operations/ReviewUnknownOperation.vue';
import ReviewAddUserGroup from './user-groups/ReviewAddUserGroup.vue';

const props = withDefaults(
  defineProps<{
    proposal: Proposal;
    details: ProposalDetails;
    loading?: boolean;
  }>(),
  {
    loading: false,
  },
);

const titleTooltip = ref(false);

const componentsMap: {
  [key in KeysOfUnion<ProposalOperation>]: Component;
} = {
  AddUserGroup: ReviewAddUserGroup,
  RemoveUserGroup: ReviewUnknownOperation,
  EditUserGroup: ReviewUnknownOperation,
  AddUser: ReviewUnknownOperation,
  EditUser: ReviewUnknownOperation,
  AddAccount: ReviewUnknownOperation,
  EditAccount: ReviewUnknownOperation,
  AddAccessPolicy: ReviewUnknownOperation,
  RemoveAccessPolicy: ReviewUnknownOperation,
  EditAccessPolicy: ReviewUnknownOperation,
  AddProposalPolicy: ReviewUnknownOperation,
  EditProposalPolicy: ReviewUnknownOperation,
  RemoveProposalPolicy: ReviewUnknownOperation,
  Transfer: ReviewUnknownOperation,
  ChangeCanister: ReviewUnknownOperation,
  AddAddressBookEntry: ReviewUnknownOperation,
  EditAddressBookEntry: ReviewUnknownOperation,
  RemoveAddressBookEntry: ReviewUnknownOperation,
};

defineEmits<{
  (event: 'approve', reason?: string): void;
  (event: 'reject', reason?: string): void;
}>();

const detailView = computed<{
  component: Component;
  operation: ProposalOperation[keyof ProposalOperation];
} | null>(() => {
  const keys = Object.keys(componentsMap) as Array<keyof ProposalOperation>;
  for (const key of keys) {
    if (key in props.proposal.operation && key in componentsMap) {
      return {
        component: componentsMap[key],
        operation: props.proposal.operation[key],
      };
    }
  }

  return null;
});

const proposalType = computed(() => {
  const keys = Object.keys(componentsMap) as KeysOfUnion<ProposalOperation>[];
  for (const key of keys) {
    if (key in props.proposal.operation) {
      return key.toLowerCase();
    }
  }

  return 'unknown';
});
</script>
