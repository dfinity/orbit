<template>
  <VRow v-if="isViewMode">
    <VCol cols="12" class="py-2 text-body-2 text-medium-emphasis">
      <template v-if="variantIs(model.auth_scope, 'Public')">
        {{ $t('permissions.allow.public') }}
      </template>
      <template v-else-if="variantIs(model.auth_scope, 'Authenticated')">
        {{ $t('permissions.allow.authenticated') }}
      </template>
      <template v-else-if="specifiedUsersHaveAccess">
        <p class="mb-1">{{ $t('permissions.allow.restricted') }}</p>

        <UsersAndGroupsAutocomplete
          v-model="permittedUsersModel"
          variant="plain"
          menu-icon=""
          :readonly="true"
          :density="props.density"
          :placeholder="$t('permissions.restrict_permitted_users')"
          :no-data-text="$t('permissions.no_users_found')"
          multiple
        />
      </template>
      <template v-else>
        <p class="mb-1">{{ $t('permissions.allow.no_access') }}</p>
      </template>
    </VCol>
  </VRow>
  <VRow v-else>
    <VCol cols="12" class="py-2 pl-0">
      <VRadioGroup v-model="displayScope" hide-details>
        <VRadio :disabled="isViewMode" :value="AllowAuthScope.NoOne">
          <template #label>
            {{ $t('permissions.allow.no_access') }}
          </template>
        </VRadio>
        <VRadio :disabled="isViewMode" :value="AllowAuthScope.Public">
          <template #label>
            {{ $t('permissions.allow.public') }}
          </template>
        </VRadio>
        <VRadio :disabled="isViewMode" :value="AllowAuthScope.LoggedIn">
          <template #label>
            {{ $t('permissions.allow.authenticated') }}
          </template>
        </VRadio>
        <VRadio :disabled="isViewMode" :value="AllowAuthScope.SpecifiedUsers">
          <template #label>
            {{ $t('permissions.allow.restricted') }}
          </template>
        </VRadio>
      </VRadioGroup>
    </VCol>
    <template v-if="displayScope === AllowAuthScope.SpecifiedUsers">
      <VCol cols="12" class="py-0">
        <UsersAndGroupsAutocomplete
          v-model="permittedUsersModel"
          :variant="props.variant"
          :readonly="isViewMode"
          :density="props.density"
          :placeholder="$t('permissions.restrict_permitted_users')"
          :no-data-text="$t('permissions.no_users_found')"
          :rules="[requiredRule]"
          hide-details="auto"
          multiple
        />
      </VCol>
    </template>
  </VRow>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref } from 'vue';
import { VCol, VRadio, VRadioGroup, VRow } from 'vuetify/components';
import { Allow } from '~/generated/station/station.did';
import { requiredRule } from '~/utils/form.utils';
import { variantIs } from '~/utils/helper.utils';
import UsersAndGroupsAutocomplete, {
  UserAndGroupsAutocompleteModel,
} from './UsersAndGroupsAutocomplete.vue';

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

const permittedUsersModel = computed({
  get: () =>
    [
      ...model.value.user_groups.map(userGroupId => ({ type: 'group', id: userGroupId })),
      ...model.value.users.map(userId => ({ type: 'user', id: userId })),
    ] as UserAndGroupsAutocompleteModel[],
  set: value => {
    model.value = {
      ...model.value,
      users: value.filter(item => item.type === 'user').map(item => item.id),
      user_groups: value.filter(item => item.type === 'group').map(item => item.id),
    };
  },
});

const isViewMode = computed(() => props.mode === 'view');

const emit = defineEmits<{
  (event: 'update:modelValue', payload: Allow): void;
  (event: 'submit', payload: Allow): void;
}>();

enum AllowAuthScope {
  Public = 'public',
  LoggedIn = 'logged_in',
  SpecifiedUsers = 'specified_users',
  NoOne = 'no_one',
}

// Lets the user select specific users when the scope is changed to Restricted
const selectingUsers = ref(false);

const displayScope = computed({
  get: () => {
    if (variantIs(model.value.auth_scope, 'Public')) {
      return AllowAuthScope.Public;
    }
    if (variantIs(model.value.auth_scope, 'Authenticated')) {
      return AllowAuthScope.LoggedIn;
    }
    if (
      (variantIs(model.value.auth_scope, 'Restricted') &&
        (model.value.users.length > 0 || model.value.user_groups.length > 0)) ||
      selectingUsers.value
    ) {
      return AllowAuthScope.SpecifiedUsers;
    }

    return AllowAuthScope.NoOne;
  },
  set: authScope => {
    let scope: Allow['auth_scope'] = { Restricted: null };

    // Reset user selection when scope is changed
    selectingUsers.value = authScope === AllowAuthScope.SpecifiedUsers;

    if (authScope === AllowAuthScope.Public) {
      scope = { Public: null };
    } else if (authScope === AllowAuthScope.LoggedIn) {
      scope = { Authenticated: null };
    }

    model.value = {
      auth_scope: scope,
      // Reset user_groups and users when scope is changed to avoid inconsistent variants
      users: [],
      user_groups: [],
    };
  },
});

const specifiedUsersHaveAccess = computed(
  () =>
    variantIs(model.value.auth_scope, 'Restricted') &&
    (model.value.users.length > 0 || model.value.user_groups.length > 0),
);

onMounted(() => {
  selectingUsers.value = displayScope.value === AllowAuthScope.SpecifiedUsers;
});
</script>
