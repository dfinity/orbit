<template>
  <VForm ref="form" @submit.prevent="submit">
    <VRadioGroup v-model="model" class="everyone_radio_group">
      <VRadio
        :disabled="isViewMode"
        :label="$t('permissions.allow.restricted')"
        :value="AuthScopeEnum.Restrictred"
      />
      <VRadio
        :disabled="isViewMode"
        :label="$t('permissions.allow.authenticated')"
        :value="AuthScopeEnum.Authenticated"
      />
      <VRadio
        :disabled="isViewMode"
        :label="$t('permissions.allow.public')"
        :value="AuthScopeEnum.Public"
      />
    </VRadioGroup>
  </VForm>
</template>

<script lang="ts" setup>
import { computed, ref, toRefs, watch } from 'vue';
import { AuthScopeEnum } from '~/types/permissions.types';
import { VFormValidation } from '~/types/helper.types';

export type EveryoneFormProps = {
  modelValue: AuthScopeEnum;
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
