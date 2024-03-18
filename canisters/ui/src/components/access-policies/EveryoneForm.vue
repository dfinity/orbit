<template>
  <VForm ref="form" @submit.prevent="submit">
    <VRadioGroup
      v-model="model"
      :label="$t('access_policies.allow.everyone_edit_label')"
      class="everyone_radio_group"
    >
      <VRadio
        :disabled="isViewMode"
        :label="$t('access_policies.allow.notset')"
        :value="AccessPolicyForAllUsers.NotSet"
      />
      <VRadio
        :disabled="isViewMode"
        :label="$t('access_policies.allow.authenticated')"
        :value="AccessPolicyForAllUsers.AuthenticationRequired"
      />
      <VRadio
        :disabled="isViewMode"
        :label="$t('access_policies.allow.anyone')"
        :value="AccessPolicyForAllUsers.Public"
      />
    </VRadioGroup>
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, toRefs, watch } from 'vue';
import { AccessPolicyForAllUsers } from '~/types/access-policies.types';
import { VFormValidation } from '~/types/helper.types';

export type EveryoneFormProps = {
  modelValue: AccessPolicyForAllUsers;
  valid?: boolean;
  mode?: 'view' | 'edit';
};

const props = withDefaults(defineProps<EveryoneFormProps>(), {
  valid: true,
  mode: 'edit',
});

const reactiveProps = toRefs(props);
const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));
const isViewMode = computed(() => reactiveProps.mode.value === 'view');

const emit = defineEmits<{
  (event: 'update:modelValue', payload: EveryoneFormProps['modelValue']): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: EveryoneFormProps['modelValue']): void;
}>();

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const model = computed({
  get: () => reactiveProps.modelValue.value,
  set: value => emit('update:modelValue', value),
});

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', model.value);
  }
};
</script>

<style lang="scss">
.everyone_radio_group.v-radio-group {
  & > .v-input__control > .v-label + .v-selection-control-group {
    padding-inline-start: 0px;
  }

  & > .v-input__control > .v-label {
    margin-inline-start: 0px;
  }
}
</style>
