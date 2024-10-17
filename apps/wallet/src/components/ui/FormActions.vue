<template>
  <div class="d-flex pa-3">
    <VSpacer />
    <VBtn
      :disabled="!edited || !valid"
      :loading="submitting"
      color="primary"
      variant="elevated"
      data-test-id="submit-btn"
      @click="submit"
    >
      {{ $t('terms.save') }}
    </VBtn>
  </div>
</template>
<script lang="ts" setup>
import { toRefs } from 'vue';
import { VBtn, VSpacer } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    valid: boolean;
    submitting: boolean;
    edited: boolean;
    submit: () => void;
  }>(),
  {
    valid: false,
    submitting: false,
    edited: false,
    submit: () => ({}),
  },
);

const { edited, valid, submitting, submit } = toRefs(props);
</script>
