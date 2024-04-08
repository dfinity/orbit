<template>
  <div class="mx-auto w-50 mt-16">
    <VBtn variant="flat" @click="emit('back')" :disabled="working">
      <VIcon :icon="mdiChevronLeft" size="x-large"></VIcon>
      {{ $t('terms.back') }}</VBtn
    >
    <VForm ref="form" @submit.prevent="addNewWallet" class="mt-12">
      <h2 class="text-h4 mb-6">{{ $t('pages.initialization.join_wallet_title') }}</h2>
      <p class="text-body-1 mb-6">
        {{ $t('pages.initialization.join_wallet_body') }}
      </p>

      <VTextField
        v-model="canisterId"
        variant="outlined"
        :rules="[requiredRule, validCanisterId]"
        :label="$t('pages.initialization.join_wallet_canister_id')"
        data-test-id="join-wallet-form-canister-id"
        :disabled="working"
      />

      <VTextField
        v-model.trim="name"
        :label="$t('pages.initialization.join_wallet_name')"
        data-test-id="join-wallet-form-name"
        variant="outlined"
        :disabled="working"
      />

      <div class="mt-6">
        <VBtn
          color="primary"
          type="submit"
          @click="addNewWallet"
          :loading="working"
          :disabled="working || !isFormValid"
          >{{ $t('pages.initialization.join_wallet') }}</VBtn
        >
      </div>
    </VForm>
  </div>
</template>

<script setup lang="ts">
import { mdiChevronLeft } from '@mdi/js';
import { VBtn, VForm, VTextField } from 'vuetify/components';
import { VFormValidation } from '~/types/helper.types';
import { useSessionStore } from '~/stores/session.store';
import { ref } from 'vue';
import { computed } from 'vue';
import { requiredRule, validCanisterId } from '~/utils/form.utils';
import { useRouter } from 'vue-router';
import { defaultHomeRoute } from '~/configs/routes.config';

const session = useSessionStore();
const router = useRouter();

const working = ref(false);
const canisterId = ref('');
const name = ref('');

const form = ref<VFormValidation | null>(null);
const isFormValid = computed(() => (form.value ? form.value.isValid : false));

const emit = defineEmits<{
  (event: 'back', payload: void): void;
}>();

async function addNewWallet() {
  if (working.value) {
    return;
  }

  const { valid } = form.value ? await form.value.validate() : { valid: false };

  if (valid) {
    working.value = true;
    try {
      await session.addWallet(canisterId.value, name.value);
      await router.push({ name: defaultHomeRoute });
    } catch {}
    working.value = false;
  }
}
</script>
