<template>
  <div class="d-flex ga-4 flex-column">
    <div class="d-flex ga-2">
      <VBtn :active="isAny" variant="outlined" size="small" @click="setSelectionMode('Any')">
        {{ $t('terms.all') }}
      </VBtn>
      <VBtn :active="isId" variant="outlined" size="small" @click="setSelectionMode('Id')">
        {{ $t('terms.subset') }}
      </VBtn>
    </div>
    <AccountAutocomplete
      v-if="isId"
      v-model="idsModel"
      :label="$t('app.search_accounts')"
      variant="underlined"
      density="comfortable"
    />
  </div>
</template>
<script setup lang="ts">
import { computed, toRefs } from 'vue';
import AccountAutocomplete from '~/components/inputs/AccountAutocomplete.vue';
import { TransferSpecifier } from '~/generated/wallet/wallet.did';
import { variantIs } from '~/utils/helper.utils';

const input = withDefaults(
  defineProps<{
    modelValue?: TransferSpecifier;
  }>(),
  {
    modelValue: () => ({ account: { Any: null } }),
  },
);

const props = toRefs(input);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: TransferSpecifier): void;
}>();

const model = computed({
  get: () => props.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const isAny = computed(() => variantIs(model.value.account, 'Any'));
const isId = computed(() => variantIs(model.value.account, 'Id'));

const idsModel = computed({
  get: () => (variantIs(model.value.account, 'Id') ? model.value.account.Id : []),
  set: value => {
    if (variantIs(model.value.account, 'Id')) {
      model.value.account.Id = value;
    }
  },
});

const setSelectionMode = (variant: 'Any' | 'Id'): void => {
  if (variantIs(model.value.account, variant)) {
    return;
  }

  if (variant === 'Any') {
    model.value.account = { Any: null };
    return;
  }

  if (variant === 'Id') {
    model.value.account = { Id: [] };
    return;
  }
};
</script>
