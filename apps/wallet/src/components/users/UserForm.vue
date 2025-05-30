<template>
  <VForm ref="form" @submit.prevent="submit">
    <VTextField
      v-if="model.id"
      v-model="model.id"
      name="id"
      :label="$t('terms.id')"
      variant="plain"
      density="compact"
      readonly
    />

    <DiffView :before-value="currentUser?.name" :after-value="model.name">
      <template #default="{ value, diffMode }">
        <VTextField
          :name="diffMode === 'before' ? 'name-before' : 'name'"
          :model-value="value"
          :label="$t('terms.name')"
          density="comfortable"
          :rules="diffMode === 'before' ? [] : [maxLengthRule(50, $t('terms.name'))]"
          :variant="isViewMode ? 'plain' : 'filled'"
          :readonly="isViewMode || diffMode === 'before'"
          @update:model-value="val => diffMode === 'after' && (name = val)"
        />
      </template>
    </DiffView>
    <DiffView :before-value="statusBefore" :after-value="status">
      <template #default="{ value, diffMode }">
        <VAutocomplete
          :model-value="value"
          :name="diffMode === 'before' ? 'status-before' : 'status'"
          :label="$t('terms.status')"
          density="comfortable"
          :items="statusItems"
          :rules="[requiredRule]"
          chips
          :variant="isViewMode ? 'plain' : 'filled'"
          :readonly="isViewMode || diffMode === 'before'"
          @update:model-value="val => diffMode === 'after' && (status = val)"
        />
      </template>
    </DiffView>
    <VCheckbox
      v-if="
        (userWasActive && status === UserStatusType.Inactive) ||
        model.cancelPendingRequests !== undefined
      "
      v-model="model.cancelPendingRequests"
      :label="$t('app.user_cancel_pending_requests')"
      density="comfortable"
      :variant="isViewMode ? 'plain' : 'filled'"
      :readonly="isViewMode"
    />

    <DiffView :before-value="userGroupsBefore" :after-value="userGroups">
      <template #default="{ value, diffMode }">
        <UserGroupAutocomplete
          :model-value="value"
          :name="diffMode === 'before' ? 'groups-before' : 'groups'"
          density="comfortable"
          :label="$t('terms.user_groups')"
          :variant="isViewMode ? 'plain' : 'filled'"
          :readonly="isViewMode || diffMode === 'before'"
          :rules="[requiredRule]"
          chips
          multiple
          @update:model-value="val => diffMode === 'after' && (userGroups = val)"
        />
      </template>
    </DiffView>

    <DiffView :before-value="identitiesBefore" :after-value="identities">
      <template #default="{ value, diffMode }">
        <VAutocomplete
          ref="identitiesInput"
          :model-value="value"
          :name="diffMode === 'before' ? 'identities-before' : 'identities'"
          density="comfortable"
          :label="$t('terms.identities')"
          :variant="isViewMode ? 'plain' : 'filled'"
          :readonly="isViewMode"
          :rules="[requiredRule]"
          :items="identities"
          chips
          multiple
          @update:model-value="val => diffMode === 'after' && (identities = val)"
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
      </template>
    </DiffView>
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
import {
  VAlert,
  VAutocomplete,
  VBtn,
  VCheckbox,
  VForm,
  VSpacer,
  VTextField,
} from 'vuetify/components';
import { variantIs } from '~/utils/helper.utils';
import DiffView from '../requests/DiffView.vue';

const props = withDefaults(
  defineProps<{
    modelValue: Partial<User & { cancelPendingRequests?: boolean }>;
    currentUser?: User;
    valid?: boolean;
    triggerSubmit?: boolean;
    mode?: 'view' | 'edit';
  }>(),
  {
    valid: true,
    triggerSubmit: false,
    mode: 'edit',
    currentUser: undefined,
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Partial<User>): void;
  (event: 'update:triggerSubmit', payload: boolean): void;
  (event: 'valid', payload: boolean): void;
  (event: 'submit', payload: Partial<User>): void;
}>();

const userWasActive = ref(
  props.modelValue.status ? variantIs(props.modelValue.status, 'Active') : false,
);
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

const statusBefore = computed({
  get: () =>
    props.currentUser?.status ? fromUserStatusVariantToEnum(props.currentUser.status) : undefined,
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

const identitiesBefore = computed({
  get: () => props.currentUser?.identities?.map(i => i.toText()),
  set: () => {}, // noop, readonly field
});

const userGroups = computed({
  get: () => model.value.groups?.map(g => g.id) ?? [],
  set: value => {
    model.value.groups = value?.map(id => ({ id, name: '' })) ?? [];
  },
});

const userGroupsBefore = computed({
  get: () => props.currentUser?.groups?.map(g => g.id),
  set: () => {}, // noop, readonly field
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
