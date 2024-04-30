<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      disabled
    />
    <VTextField
      v-model="name"
      name="name"
      :label="$t('terms.name')"
      density="comfortable"
      :rules="[maxLengthRule(50, $t('terms.name'))]"
      :variant="isViewMode ? 'plain' : 'filled'"
      :disabled="isViewMode"
    />
    <VAutocomplete
      v-model="status"
      name="status"
      :label="$t('terms.status')"
      density="comfortable"
      :items="statusItems"
      :rules="[requiredRule]"
      chips
      :variant="isViewMode ? 'plain' : 'filled'"
      :disabled="isViewMode"
    />
    <UserGroupAutocomplete
      v-model="userGroups"
      name="groups"
      density="comfortable"
      :label="$t('terms.user_groups')"
      :variant="isViewMode ? 'plain' : 'filled'"
      :disabled="isViewMode"
      :rules="[requiredRule]"
      chips
      multiple
    />
    <VAutocomplete
      ref="identitiesInput"
      v-model="identities"
      name="identities"
      density="comfortable"
      :label="$t('terms.identities')"
      :variant="isViewMode ? 'plain' : 'filled'"
      :disabled="isViewMode"
      :rules="[requiredRule]"
      :items="identities"
      chips
      multiple
    >
      <template v-if="!isViewMode" #append>
        <ActionBtn
          v-model="addNewPrincipalModel"
          :title="$t('app.add_new_identity')"
          :icon="mdiPlus"
          variant="tonal"
          :submit="
            newPrincipal => {
              if (newPrincipal.model) {
                if (identities?.includes(newPrincipal.model)) {
                  app.sendNotification({
                    type: 'warning',
                    message: $t('app.principal_already_added'),
                  });

                  return;
                }

                // an assignment is necessary to trigger the reactivity
                identities = [...identities, newPrincipal.model];

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
                <VAlert type="warning" variant="tonal" density="compact" class="mb-4">
                  {{ $t('app.user_associate_identity_warning') }}
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
import { Principal } from '@dfinity/principal';
import { mdiPlus } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import UserGroupAutocomplete from '~/components/inputs/UserGroupAutocomplete.vue';
import { User } from '~/generated/station/station.did';
import { fromUserStatusEnumToVariant, fromUserStatusVariantToEnum } from '~/mappers/users.mapper';
import { useAppStore } from '~/stores/app.store';
import { VFormValidation } from '~/types/helper.types';
import { UserStatusType } from '~/types/station.types';
import { maxLengthRule, requiredRule } from '~/utils/form.utils';
import AddPrincipalForm from './AddPrincipalForm.vue';
import { VAlert, VAutocomplete, VBtn, VForm, VSpacer, VTextField } from 'vuetify/components';

const props = withDefaults(
  defineProps<{
    modelValue: Partial<User>;
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
  (event: 'update:modelValue', payload: Partial<User>): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: Partial<User>): void;
}>();

const i18n = useI18n();
const app = useAppStore();
const form = ref<VFormValidation | null>(null);
const identitiesInput = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const model = computed(() => props.modelValue);
watch(model.value, newValue => emit('update:modelValue', newValue), { deep: true });

watch(
  () => isFormValid.value,
  isValid => emit('valid', isValid ?? false),
);

const status = computed({
  get: () => (model.value.status ? fromUserStatusVariantToEnum(model.value.status) : undefined),
  set: value => {
    model.value.status = value ? fromUserStatusEnumToVariant(value) : undefined;
  },
});

const name = computed({
  get: () => model.value.name,
  set: value => {
    model.value.name = !value ? '' : value;
  },
});

const identities = computed({
  get: () => model.value.identities?.map(i => i.toText()) ?? [],
  set: value => {
    model.value.identities = value?.map(i => Principal.fromText(i)) ?? [];
  },
});

const userGroups = computed({
  get: () => model.value.groups?.map(g => g.id) ?? [],
  set: value => {
    model.value.groups = value?.map(id => ({ id, name: '' })) ?? [];
  },
});

const statusItems = computed(() =>
  Object.values(UserStatusType).map(status => ({
    title: i18n.t(`app.user_status_${status.toLowerCase()}`),
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
    emit('submit', model.value);
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
