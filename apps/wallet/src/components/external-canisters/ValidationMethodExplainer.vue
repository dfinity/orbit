<template>
  <i18n-t
    keypath="external_canisters.call_configuration.card_validation_method_description"
    scope="global"
  >
    <template #method>
      <VChip label size="small" density="comfortable" data-test-id="target-method">
        <TextOverflow :max-length="64" :text="props.validationMethod" />
      </VChip>
    </template>
    <template #canister>
      <span>
        <VChip label size="small" density="comfortable" data-test-id="target-canister">
          {{
            props.validationCanisterId.toText() === props.selfCanisterId?.toText()
              ? $t('terms.self').toLowerCase()
              : props.validationCanisterId.toText()
          }}
        </VChip>

        <VBtn
          size="x-small"
          variant="text"
          density="comfortable"
          :icon="mdiContentCopy"
          @click="
            copyToClipboard({
              textToCopy: props.validationCanisterId.toText(),
              sendNotification: true,
            })
          "
        />
      </span>
    </template>
  </i18n-t>
</template>

<script lang="ts" setup>
import { mdiContentCopy } from '@mdi/js';
import { copyToClipboard } from '~/utils/app.utils';
import TextOverflow from '../TextOverflow.vue';
import { Principal } from '@dfinity/principal';

const props = withDefaults(
  defineProps<{
    validationMethod: string;
    validationCanisterId: Principal;
    selfCanisterId?: Principal;
  }>(),
  {
    selfCanisterId: undefined,
  },
);
</script>
