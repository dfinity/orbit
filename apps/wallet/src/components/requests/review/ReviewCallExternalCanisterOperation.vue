<template>
  <VList density="compact" class="w-100">
    <VListItem v-if="canisterId !== review.canisterId.toText()" class="px-0 mx-0">
      <VListItemTitle class="font-weight-bold text-body-2">
        {{ $t('terms.canister_id') }}
      </VListItemTitle>
      <VListItemSubtitle>
        {{ review.canisterId.toText() }}

        <VBtn
          size="x-small"
          variant="text"
          :icon="mdiContentCopy"
          @click="
            copyToClipboard({
              textToCopy: review.canisterId.toText(),
              sendNotification: true,
            })
          "
        />
      </VListItemSubtitle>
    </VListItem>
    <VListItem class="px-0 mx-0">
      <VListItemTitle class="font-weight-bold text-body-2">
        {{ $t('external_canisters.perform_call.method_name') }}
      </VListItemTitle>
      <VListItemSubtitle>
        {{ review.methodName }}
      </VListItemSubtitle>
    </VListItem>
    <VListItem v-if="props.fullReviewContext && review.cycles" class="px-0 mx-0">
      <VListItemTitle class="font-weight-bold text-body-2">
        {{ $t('external_canisters.perform_call.attached_cycles') }}
      </VListItemTitle>
      <VListItemSubtitle>
        {{ toCyclesUnit(review.cycles, CyclesUnit.Trillion) }} {{ $t('cycles.units.tc') }}
      </VListItemSubtitle>
    </VListItem>
    <VListItem v-if="props.fullReviewContext && review.validationMethod" class="px-0 mx-0">
      <VListItemTitle class="font-weight-bold text-body-2">
        {{ $t('external_canisters.perform_call.validation_method') }}
      </VListItemTitle>
      <VListItemSubtitle>
        <ValidationMethodExplainer
          :validation-method="review.validationMethod.method_name"
          :validation-canister-id="review.validationMethod.canister_id"
          :self-canister-id="review.canisterId"
        />
      </VListItemSubtitle>
    </VListItem>
    <VListItem v-if="review.argChecksum" class="px-0 mx-0">
      <VListItemTitle class="font-weight-bold text-body-2">
        {{ $t('external_canisters.perform_call.argument_checksum') }}
      </VListItemTitle>
      <VListItemSubtitle>
        <TextOverflow :text="review.argChecksum" :max-length="props.fullReviewContext ? 64 : 24" />

        <VBtn
          size="x-small"
          variant="text"
          :icon="mdiContentCopy"
          @click="
            copyToClipboard({
              textToCopy: review.argChecksum,
              sendNotification: true,
            })
          "
        />
      </VListItemSubtitle>
    </VListItem>
    <VListItem v-if="props.fullReviewContext && review.argValidationRendering" class="px-0 mx-0">
      <VListItemTitle class="font-weight-bold text-body-2">
        {{ $t('external_canisters.perform_call.validated_argument') }}
      </VListItemTitle>
      <VListItemSubtitle>
        <VTextarea :model-value="review.argValidationRendering" rows="1" readonly hide-details />
      </VListItemSubtitle>
    </VListItem>
    <VListItem v-if="props.fullReviewContext && review.argHex" class="px-0 mx-0">
      <VListItemTitle class="font-weight-bold text-body-2">
        {{ $t('external_canisters.perform_call.argument') }}
        ({{ $t('external_canisters.wasm_args_formats.hex').toLowerCase() }})

        <VBtn
          size="x-small"
          variant="text"
          :icon="mdiContentCopy"
          @click="
            copyToClipboard({
              textToCopy: review.argHex,
              sendNotification: true,
            })
          "
        />
      </VListItemTitle>
      <VListItemSubtitle>
        <VTextarea :model-value="review.argHex" rows="2" readonly hide-details />
      </VListItemSubtitle>
    </VListItem>
    <VListItem v-if="review.replyHex" class="px-0 mx-0">
      <VListItemTitle class="font-weight-bold text-body-2">
        {{ $t('external_canisters.perform_call.reply_received') }}
        ({{ $t('external_canisters.wasm_args_formats.hex').toLowerCase() }})

        <VBtn
          size="x-small"
          variant="text"
          :icon="mdiContentCopy"
          @click="
            copyToClipboard({
              textToCopy: review.replyHex,
              sendNotification: true,
            })
          "
        />
      </VListItemTitle>
      <VListItemSubtitle v-if="props.fullReviewContext">
        <VTextarea :model-value="review.replyHex" rows="2" readonly hide-details />
      </VListItemSubtitle>
    </VListItem>
  </VList>
</template>

<script setup lang="ts">
import { mdiContentCopy } from '@mdi/js';
import { CanisterCallReviewContext } from '~/components/external-canisters/external-canisters.types';
import ValidationMethodExplainer from '~/components/external-canisters/ValidationMethodExplainer.vue';
import TextOverflow from '~/components/TextOverflow.vue';
import { useExternalCanisterProvider } from '~/composables/external-canisters.composable';
import { toCyclesUnit } from '~/mappers/cycles.mapper';
import { CyclesUnit } from '~/types/app.types';
import { copyToClipboard } from '~/utils/app.utils';

const props = withDefaults(
  defineProps<{
    review: CanisterCallReviewContext;
    fullReviewContext?: boolean;
  }>(),
  {
    fullReviewContext: false,
  },
);

const { canisterId } = useExternalCanisterProvider();
</script>
