<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="modelValue.id"
      v-model="modelValue.id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      readonly
    />
    <VTextField
      v-model="modelValue.name"
      :label="$t('terms.name')"
      variant="underlined"
      :rules="rules.name"
    />
    <VAutocomplete
      v-model="status"
      :label="$t('terms.status')"
      variant="underlined"
      :items="statusItems"
      chips
    />
    <VAutocomplete
      v-model="modelValue.groups"
      :label="$t('terms.user_groups')"
      variant="underlined"
      :rules="rules.groups"
      :items="userGroups"
      chips
      multiple
    />
    <VAutocomplete
      v-model="modelValue.identities"
      :label="$t('terms.principal')"
      variant="underlined"
      :rules="rules.principals"
      :items="modelValue.identities ?? []"
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
              if (newPrincipal.model) {
                if (!modelValue.identities) {
                  modelValue.identities = [];
                }

                if (modelValue.identities.includes(newPrincipal.model)) {
                  app.sendNotification({
                    type: 'warning',
                    message: $t('app.principal_already_added'),
                  });

                  return;
                }

                modelValue.identities.push(newPrincipal.model);
              }
            }
          "
          data-testid="add-principal-btn"
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
import { computed, ref, watch } from 'vue';
import { fromUserStatusVariantToEnum, fromUserStatusEnumToVariant } from '~/mappers/users.mapper';
import { UserInput, UserStatusType } from '~/types';
import { i18n } from '~/ui/modules/i18n';
import { FormValidationRules, VFormValidation } from '~/ui/types';
import { maxLengthRule, requiredRule } from '~/ui/utils';
import { useWalletStore } from '~/ui/stores/wallet';
import ActionBtn from '~/ui/components/buttons/ActionBtn.vue';
import AddPrincipalForm from '~/ui/components/forms/AddPrincipalForm.vue';
import { useAppStore } from '~/ui/stores/app';

const wallet = useWalletStore();
const app = useAppStore();
const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));
const rules: {
  name: FormValidationRules;
  groups: FormValidationRules;
  principals: FormValidationRules;
} = {
  name: [maxLengthRule(100, i18n.global.t('terms.name'))],
  groups: [requiredRule],
  principals: [requiredRule],
};

const props = withDefaults(
  defineProps<{
    modelValue: Partial<UserInput>;
    valid?: boolean;
  }>(),
  {
    valid: true,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<UserInput>): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: Partial<UserInput>): void;
}>();

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const modelValue = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const status = computed({
  get: () => fromUserStatusVariantToEnum(modelValue.value.status ?? { Inactive: null }),
  set: value => {
    modelValue.value.status = fromUserStatusEnumToVariant(value);
  },
});

const statusItems = computed(() =>
  Object.values(UserStatusType).map(status => ({
    title: i18n.global.t(`app.user_status_${status.toLowerCase()}`),
    value: status,
  })),
);

const userGroups = computed(() => {
  const groups = wallet.configuration.details.user_groups.map(group => ({
    title: group.name,
    value: group.id,
  }));

  modelValue.value.groups?.forEach(group => {
    if (!groups.find(g => g.value === group)) {
      groups.push({
        title: group,
        value: group,
      });
    }
  });

  return groups;
});

const submit = async () => {
  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    emit('submit', modelValue.value);
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
