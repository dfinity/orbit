<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="modelValue.id"
      v-model="modelValue.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      readonly
    />
    <VTextField
      v-model="name"
      name="name"
      :label="$t('terms.name')"
      :rules="[maxLengthRule(100, $t('terms.name'))]"
      :variant="isViewMode ? 'plain' : 'underlined'"
      :readonly="isViewMode"
    />
    <VAutocomplete
      v-model="status"
      name="status"
      :label="$t('terms.status')"
      :items="statusItems"
      chips
      :variant="isViewMode ? 'plain' : 'underlined'"
      :readonly="isViewMode"
    />
    <UserGroupAutocomplete
      v-model="modelValue.groups"
      name="groups"
      :label="$t('terms.user_groups')"
      :variant="isViewMode ? 'plain' : 'underlined'"
      :readonly="isViewMode"
      :rules="[requiredRule]"
      chips
      multiple
    />
    <VAutocomplete
      ref="identitiesInput"
      v-model="modelValue.identities"
      name="identities"
      :label="$t('terms.principal')"
      :variant="isViewMode ? 'plain' : 'underlined'"
      :readonly="isViewMode"
      :rules="[requiredRule]"
      :items="modelValue.identities"
      chips
      multiple
    >
      <template #append>
        <ActionBtn
          v-model="addNewPrincipalModel"
          :title="$t('app.add_new_principal')"
          :icon="mdiPlus"
          color="primary"
          :submit="
            newPrincipal => {
              if (!modelValue.identities) {
                modelValue.identities = [];
              }

              if (newPrincipal.model) {
                if (modelValue.identities?.includes(newPrincipal.model)) {
                  app.sendNotification({
                    type: 'warning',
                    message: $t('app.principal_already_added'),
                  });

                  return;
                }

                modelValue.identities.push(newPrincipal.model);

                identitiesInput?.validate();
              }
            }
          "
          data-test-id="add-principal-btn"
        >
          <template #default="{ model: elem, submit: addNewPrincipal }">
            <AddPrincipalForm
              v-model="elem.value.model"
              @valid="isValid => (elem.value.valid = isValid)"
              @submit="addNewPrincipal"
            >
              <template #prepend>
                <VAlert type="warning" variant="outlined" density="compact" class="mb-4">
                  {{ $t('app.user_associate_principal_warning') }}
                </VAlert>
              </template>
            </AddPrincipalForm>
          </template>
          <template #actions="{ submit: addNewPrincipal, loading: saving, model: elem }">
            <VSpacer />
            <VBtn
              :loading="saving"
              :disabled="!elem.value.valid"
              color="primary"
              variant="flat"
              @click="addNewPrincipal"
            >
              {{ $t('terms.add') }}
            </VBtn>
          </template>
        </ActionBtn>
      </template>
    </VAutocomplete>
  </VForm>
</template>

<script lang="ts" setup>
import { mdiPlus } from '@mdi/js';
import { computed, reactive, ref, watch } from 'vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import UserGroupAutocomplete from '~/components/inputs/UserGroupAutocomplete.vue';
import { fromUserStatusEnumToVariant, fromUserStatusVariantToEnum } from '~/mappers/users.mapper';
import { i18n } from '~/plugins/i18n.plugin';
import { useAppStore } from '~/stores/app.store';
import { VFormValidation } from '~/types/helper.types';
import { UserDTO, UserStatusType } from '~/types/wallet.types';
import { maxLengthRule, requiredRule } from '~/utils/form.utils';
import AddPrincipalForm from './AddPrincipalForm.vue';

const props = withDefaults(
  defineProps<{
    modelValue: Partial<UserDTO>;
    valid?: boolean;
    triggerSubmit?: boolean;
    mode?: 'view' | 'edit';
  }>(),
  {
    valid: true,
    triggerSubmit: false,
    mode: 'edit',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<UserDTO>): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: Partial<UserDTO>): void;
}>();

const app = useAppStore();
const form = ref<VFormValidation | null>(null);
const identitiesInput = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const modelValue = reactive({ ...props.modelValue });

watch(
  () => modelValue,
  value => emit('update:modelValue', value),
  { deep: true },
);

const status = computed({
  get: () => fromUserStatusVariantToEnum(props.modelValue.status ?? { Inactive: null }),
  set: value => {
    modelValue.status = fromUserStatusEnumToVariant(value);
  },
});

const name = computed({
  get: () => modelValue.name?.[0] ?? null,
  set: value => {
    modelValue.name = !value ? [] : [value];
  },
});

const statusItems = computed(() =>
  Object.values(UserStatusType).map(status => ({
    title: i18n.global.t(`app.user_status_${status.toLowerCase()}`),
    value: status,
  })),
);

watch(
  () => props.triggerSubmit,
  () => {
    if (props.triggerSubmit) {
      emit('update:triggerSubmit', false);
      submit();
    }
  },
);

const isViewMode = computed(() => props.mode === 'view');

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', modelValue);
  }
};

const addNewPrincipalModel = ref<{
  valid: boolean;
  model: string | null;
}>({
  model: null,
  valid: false,
});
</script>
