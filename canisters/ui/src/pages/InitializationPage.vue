<template>
  <PageLayout>
    <template #sidebar-nav><div /></template>
    <template #main-body>
      <div class="screens" data-test-id="split-screen">
        <TransitionGroup :name="transitionDestinationNames[initializationStep]">
          <div
            v-if="initializationStep == InitializationStep.ChooseOption"
            class="mx-auto w-50 mt-16"
          >
            <h1 class="text-h4 mb-6">{{ $t('pages.initialization.join_title') }}</h1>

            <VRadioGroup v-model="userChoice">
              <VRadio
                :label="$t('pages.initialization.option_join_existing_wallet')"
                :value="UserOptions.JoinExisting"
                data-test-id="join-existing-wallet-radio"
              ></VRadio>
              <VRadio
                :label="$t('pages.initialization.option_deploy_new_wallet')"
                :value="UserOptions.CreateNew"
                data-test-id="deploy-new-wallet-radio"
              ></VRadio>
            </VRadioGroup>

            <VBtn
              color="primary"
              class="mt-2"
              data-test-id="continue-button"
              @click="onInitializationOptionChosen"
            >
              {{ $t('terms.continue') }}
            </VBtn>
          </div>

          <JoinWallet
            v-if="initializationStep == InitializationStep.JoinWallet"
            @back="initializationStep = InitializationStep.ChooseOption"
          ></JoinWallet>
          <DeployWallet
            v-if="initializationStep == InitializationStep.DeployWallet"
            @back="initializationStep = InitializationStep.ChooseOption"
          />
        </TransitionGroup>
      </div>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import PageLayout from '~/components/PageLayout.vue';
import DeployWallet from '~/components/initialization/DeployWallet.vue';
import JoinWallet from '~/components/initialization/JoinWallet.vue';
import { VBtn, VRadio, VRadioGroup } from 'vuetify/components';
import { unreachable } from '~/utils/helper.utils';

enum UserOptions {
  JoinExisting = 'join-existing',
  CreateNew = 'create-new',
}
const userChoice = ref<UserOptions>(UserOptions.JoinExisting);

enum InitializationStep {
  ChooseOption = 'choose-option',
  DeployWallet = 'deploy-wallet',
  JoinWallet = 'join-wallet',
}
const initializationStep = ref<InitializationStep>(InitializationStep.ChooseOption);

const transitionDestinationNames: Record<InitializationStep, string> = {
  [InitializationStep.ChooseOption]: 'left',
  [InitializationStep.DeployWallet]: 'right',
  [InitializationStep.JoinWallet]: 'right',
};

function onInitializationOptionChosen() {
  if (userChoice.value === UserOptions.CreateNew) {
    initializationStep.value = InitializationStep.DeployWallet;
  } else if (userChoice.value === UserOptions.JoinExisting) {
    initializationStep.value = InitializationStep.JoinWallet;
  } else {
    unreachable(userChoice.value);
  }
}
</script>

<style scoped>
.left-enter-active,
.left-leave-active,
.right-enter-active,
.right-leave-active {
  transition: all 250ms ease;
}

.left-enter-from,
.right-enter-from {
  opacity: 0;
}

.left-leave-to,
.right-leave-to {
  opacity: 0;
}

.left-enter-from {
  transform: translateX(-100%);
}
.left-leave-to {
  transform: translateX(100%);
}

.right-enter-from {
  transform: translateX(100%);
}
.right-leave-to {
  transform: translateX(-100%);
}
.screens {
  display: grid;
  grid-template-columns: 1fr;
  grid-template-rows: 1fr;
}

.screens > * {
  grid-column: 1;
  grid-row: 1;
}
</style>
