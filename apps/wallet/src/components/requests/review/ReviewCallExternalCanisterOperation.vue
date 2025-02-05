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
    <VListItem v-if="props.fullReviewContext && review.arg" class="px-0 mx-0">
      <VListItemTitle class="font-weight-bold text-body-2">
        {{ $t('external_canisters.perform_call.argument') }}
      </VListItemTitle>
      <VListItemSubtitle class="mt-1">
        <LabeledTextDisplay :items="argDisplayItems" />
      </VListItemSubtitle>
    </VListItem>
    <VListItem v-if="review.reply" class="px-0 mx-0">
      <VListItemTitle class="font-weight-bold text-body-2">
        {{ $t('external_canisters.perform_call.reply_received') }}
        <template v-if="!props.fullReviewContext && replyHex">
          ({{ $t('external_canisters.wasm_args_formats.hex').toLowerCase() }})

          <VBtn
            size="x-small"
            variant="text"
            :icon="mdiContentCopy"
            @click="
              copyToClipboard({
                textToCopy: replyHex,
                sendNotification: true,
              })
            "
          />
        </template>
      </VListItemTitle>
      <VListItemSubtitle v-if="props.fullReviewContext">
        <LabeledTextDisplay :items="replyDisplayItems" class="mt-1" />
      </VListItemSubtitle>
    </VListItem>
  </VList>
</template>

<script setup lang="ts">
import { mdiContentCopy } from '@mdi/js';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { CanisterCallReviewContext } from '~/components/external-canisters/external-canisters.types';
import ValidationMethodExplainer from '~/components/external-canisters/ValidationMethodExplainer.vue';
import TextOverflow from '~/components/TextOverflow.vue';
import LabeledTextDisplay from '~/components/ui/LabeledTextDisplay.vue';
import { useExternalCanisterProvider } from '~/composables/external-canisters.composable';
import { toCyclesUnit } from '~/mappers/cycles.mapper';
import { CyclesUnit } from '~/types/app.types';
import { copyToClipboard } from '~/utils/app.utils';
import { arrayBufferToHex } from '~/utils/crypto.utils';
import { decode } from '~/utils/didc.utils';
import { toUint8Array } from '~/utils/helper.utils';

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
const i18n = useI18n();

const replyHex = computed(() =>
  props.review.reply ? arrayBufferToHex(toUint8Array(props.review.reply)) : undefined,
);

const replyCandid = computed(() => {
  if (!props.review.reply || !props.review.candidIdl || !replyHex.value) {
    return undefined;
  }

  try {
    return decode({
      idl: props.review.candidIdl,
      serviceMethod: props.review.methodName ?? undefined,
      input: replyHex.value,
    });
  } catch (_error) {
    return undefined;
  }
});

const argHex = computed(() =>
  props.review.arg ? arrayBufferToHex(toUint8Array(props.review.arg)) : undefined,
);

const argCandid = computed(() => {
  if (!props.review.arg || !props.review.candidIdl || !argHex.value) {
    return undefined;
  }

  try {
    return decode({
      idl: props.review.candidIdl,
      serviceMethod: props.review.methodName ?? undefined,
      input: argHex.value,
      useServiceMethodReturnType: false,
    });
  } catch (_error) {
    return undefined;
  }
});

const argDisplayItems = computed(() => {
  const items: { title: string; content: string }[] = [];

  if (argCandid.value) {
    items.push({
      title: i18n.t('external_canisters.wasm_args_formats.candid'),
      content: argCandid.value,
    });
  }

  if (argHex.value) {
    items.push({
      title: i18n.t('external_canisters.wasm_args_formats.hex'),
      content: argHex.value,
    });
  }

  return items;
});

const replyDisplayItems = computed(() => {
  const items: { title: string; content: string }[] = [];

  if (replyCandid.value) {
    items.push({
      title: i18n.t('external_canisters.wasm_args_formats.candid'),
      content: replyCandid.value,
    });
  }

  if (replyHex.value) {
    items.push({
      title: i18n.t('external_canisters.wasm_args_formats.hex'),
      content: replyHex.value,
    });
  }

  return items;
});
</script>
