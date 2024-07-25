<template>
  <VTextField
    :label="$t('terms.module_checksum')"
    :model-value="props.wasmModuleChecksum"
    name="wasm_checksum"
    variant="plain"
    density="comfortable"
    :prepend-icon="mdiPound"
    readonly
  />

  <VTextarea
    v-model="comment"
    name="comment"
    :label="$t(`requests.comment_optional`)"
    :prepend-icon="mdiComment"
    variant="filled"
    density="comfortable"
    auto-grow
  />
</template>

<script lang="ts" setup>
import { mdiComment, mdiPound } from '@mdi/js';
import { computed } from 'vue';
import { VTextarea, VTextField } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    wasmModuleChecksum: string;
    comment?: string;
  }>(),
  {
    comment: undefined,
  },
);

const comment = computed({
  get: () => props.comment,
  set: value => emit('update:comment', value),
});

const emit = defineEmits<{
  (event: 'update:comment', payload?: string): void;
}>();
</script>
