<template>
  <div class="d-flex ga-4 flex-column">
    <div v-if="!props.readonly.value" class="d-flex ga-2">
      <VBtn
        :active="isAny"
        :disabled="props.disabled.value"
        variant="flat"
        :color="isAny ? 'primary' : undefined"
        size="small"
        @click="setSelectionMode('Any')"
      >
        {{ $t('terms.all') }}
      </VBtn>
      <VBtn
        :active="isId"
        :disabled="props.disabled.value"
        :color="isId ? 'primary' : undefined"
        variant="flat"
        size="small"
        @click="setSelectionMode('Id')"
      >
        {{ $t('terms.subset') }}
      </VBtn>
    </div>
    <AccountAutocomplete
      v-if="isId"
      v-model="idsModel"
      :label="$t('terms.accounts')"
      variant="underlined"
      density="comfortable"
      multiple
      :disabled="props.disabled.value || props.readonly.value"
    />
  </div>
</template>
<script setup lang="ts">
import { computed, toRefs } from 'vue';
import { VBtn } from 'vuetify/components';
import AccountAutocomplete from '~/components/inputs/AccountAutocomplete.vue';
import { CommonSpecifier } from '~/generated/wallet/wallet.did';
import { variantIs } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    modelValue?: CommonSpecifier;
    disabled?: boolean;
    readonly?: boolean;
  }>(),
  {
    modelValue: () => ({ Any: null }),
    disabled: false,
    readonly: false,
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CommonSpecifier): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const isAny = computed(() => variantIs(model.value, 'Any'));
const isId = computed(() => variantIs(model.value, 'Id'));

const idsModel = computed({
  get: () => (variantIs(model.value, 'Id') ? model.value.Id : []),
  set: value => {
    if (variantIs(model.value, 'Id')) {
      model.value.Id = value;
    }
  },
});

const setSelectionMode = (variant: 'Any' | 'Id'): void => {
  if (variantIs(model.value, variant)) {
    return;
  }

  if (variant === 'Any') {
    model.value = { Any: null };
    return;
  }

  if (variant === 'Id') {
    model.value = { Id: [] };
    return;
  }
};
</script>
