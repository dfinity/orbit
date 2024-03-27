<template>
  <VRow>
    <VCol cols="12" class="py-2 pl-0">
      <VRadioGroup v-model="model.auth_scope" inline hide-details>
        <VRadio
          :disabled="isViewMode"
          :label="$t('access_policies.allow.restricted')"
          :value="{ Restricted: null }"
        />
        <VRadio
          :disabled="isViewMode"
          :label="$t('access_policies.allow.authenticated')"
          :value="{ Authenticated: null }"
        />
        <VRadio
          :disabled="isViewMode"
          :label="$t('access_policies.allow.public')"
          :value="{ Public: null }"
        />
      </VRadioGroup>
    </VCol>
    <VCol cols="12" md="6" class="py-0">
      <UserGroupAutocomplete
        v-if="isRestrictedScope"
        v-model="model.user_groups"
        :variant="props.variant"
        multiple
        :disabled="isViewMode"
        :density="props.density"
      />
    </VCol>
    <VCol cols="12" md="6" class="py-0">
      <UserAutocomplete
        v-if="isRestrictedScope"
        v-model="model.users"
        :variant="props.variant"
        multiple
        chips
        :disabled="isViewMode"
        :density="props.density"
      />
    </VCol>
  </VRow>
</template>

<script lang="ts" setup>
import { computed, watch } from 'vue';
import { VRadioGroup } from 'vuetify/components';
import UserAutocomplete from '~/components/inputs/UserAutocomplete.vue';
import UserGroupAutocomplete from '~/components/inputs/UserGroupAutocomplete.vue';
import { Allow } from '~/generated/wallet/wallet.did';
import { variantIs } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    modelValue: Allow;
    mode?: 'view' | 'edit';
    variant?: 'underlined' | 'outlined' | 'filled';
    density?: 'comfortable' | 'compact';
  }>(),
  {
    valid: true,
    mode: 'edit',
    variant: 'filled',
    density: 'comfortable',
  },
);

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const isViewMode = computed(() => props.mode === 'view');

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Allow): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: Allow): void;
}>();

const isRestrictedScope = computed(() => variantIs(model.value.auth_scope, 'Restricted'));

watch(
  () => isRestrictedScope.value,
  isRestricted => {
    // Reset user_groups and users when scope is changed from restricted to other
    if (!isRestricted) {
      model.value = {
        ...model.value,
        user_groups: [],
        users: [],
      };
    }
  },
);
</script>
