<template>
  <div data-test-id="join-station-screen">
    <VBtn variant="flat" :disabled="working" data-test-id="back-button" @click="emit('back')">
      <VIcon :icon="mdiChevronLeft" size="x-large"></VIcon>
      {{ $t('terms.back') }}</VBtn
    >
    <VForm ref="form" class="mt-12" @submit.prevent="addNewStation">
      <h2 class="text-h4 mb-6">{{ $t('pages.add_station.join_station_title') }}</h2>
      <p class="text-body-1 mb-6">
        {{ $t('pages.add_station.join_station_body') }}
      </p>

      <VTextField
        :model-value="session.principal"
        variant="plain"
        :label="$t('terms.principal')"
        readonly
        :append-inner-icon="mdiContentCopy"
        @click:append-inner="
          copyToClipboard({ textToCopy: session.principal, sendNotification: true })
        "
      />

      <VTextField
        v-model="canisterId"
        variant="outlined"
        :rules="[requiredRule, validCanisterId]"
        :label="$t('pages.add_station.join_station_canister_id')"
        data-test-id="join-station-form-canister-id"
        :disabled="working"
      />

      <VTextField
        v-model.trim="name"
        :label="$t('pages.add_station.join_station_name')"
        data-test-id="join-station-form-canister-name"
        variant="outlined"
        :disabled="working"
      />

      <div class="mt-6">
        <VBtn
          color="primary"
          type="submit"
          :loading="working"
          :disabled="working || !isFormValid"
          @click="addNewStation"
          >{{ $t('pages.add_station.join_station') }}</VBtn
        >
      </div>
    </VForm>
  </div>
</template>

<script setup lang="ts">
import { mdiChevronLeft, mdiContentCopy } from '@mdi/js';
import { VBtn, VForm, VTextField } from 'vuetify/components';
import { VFormValidation } from '~/types/helper.types';
import { useSessionStore } from '~/stores/session.store';
import { ref } from 'vue';
import { computed } from 'vue';
import { requiredRule, validCanisterId } from '~/utils/form.utils';
import { useRouter } from 'vue-router';
import { defaultHomeRoute } from '~/configs/routes.config';
import { useAppStore } from '~/stores/app.store';
import { copyToClipboard } from '~/utils/app.utils';

const session = useSessionStore();
const router = useRouter();
const app = useAppStore();

const working = ref(false);
const canisterId = ref('');
const name = ref('');

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const emit = defineEmits<{
  (event: 'back', payload: void): void;
}>();

async function addNewStation() {
  if (working.value) {
    return;
  }

  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    working.value = true;
    try {
      await session.addStation(canisterId.value, name.value);
      await router.push({ name: defaultHomeRoute });
    } catch (e: unknown) {
      app.sendErrorNotification(e);
    }
    working.value = false;
  }
}
</script>
