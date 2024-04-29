<template>
  <VContainer fluid>
    <div class="screens" data-test-id="split-screen">
      <TransitionGroup :name="transitionDestinationNames[initializationStep]">
        <VCard
          v-if="initializationStep == InitializationStep.ChooseOption"
          class="mt-16 pa-4 mx-auto w-md-75"
        >
          <VCardText>
            <h1 class="text-h4 mb-6">{{ props.title }}</h1>

            <VRadioGroup v-model="userChoice">
              <VRadio
                :label="$t('pages.add_station.option_join_existing_station')"
                :value="UserOptions.JoinExisting"
                data-test-id="join-existing-station-radio"
              />
              <VRadio
                :label="$t('pages.add_station.option_deploy_new_station')"
                :value="UserOptions.CreateNew"
                data-test-id="deploy-new-station-radio"
              />
            </VRadioGroup>

            <VBtn
              color="primary"
              class="mt-2"
              data-test-id="continue-button"
              @click="onInitializationOptionChosen"
            >
              {{ $t('terms.continue') }}
            </VBtn>
          </VCardText>
        </VCard>
        <VCard
          v-if="initializationStep == InitializationStep.JoinStation"
          class="mt-16 pa-4 mx-auto w-md-75"
        >
          <VCardText>
            <JoinStation @back="initializationStep = InitializationStep.ChooseOption" />
          </VCardText>
        </VCard>
        <VCard
          v-if="initializationStep == InitializationStep.DeployStation"
          class="mt-16 pa-4 mx-auto w-md-75"
        >
          <VCardText>
            <DeployStation @back="initializationStep = InitializationStep.ChooseOption" />
          </VCardText>
        </VCard>
      </TransitionGroup>
    </div>
  </VContainer>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import DeployStation from '~/components/add-station/DeployStation.vue';
import JoinStation from '~/components/add-station/JoinStation.vue';
import { VBtn, VCard, VCardText, VContainer, VRadio, VRadioGroup } from 'vuetify/components';
import { unreachable } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    title?: string;
  }>(),
  {
    title: '',
  },
);

enum UserOptions {
  JoinExisting = 'join-existing',
  CreateNew = 'create-new',
}
const userChoice = ref<UserOptions>(UserOptions.JoinExisting);

enum InitializationStep {
  ChooseOption = 'choose-option',
  DeployStation = 'deploy-station',
  JoinStation = 'join-station',
}
const initializationStep = ref<InitializationStep>(InitializationStep.ChooseOption);

const transitionDestinationNames: Record<InitializationStep, string> = {
  [InitializationStep.ChooseOption]: 'left',
  [InitializationStep.DeployStation]: 'right',
  [InitializationStep.JoinStation]: 'right',
};

function onInitializationOptionChosen() {
  if (userChoice.value === UserOptions.CreateNew) {
    initializationStep.value = InitializationStep.DeployStation;
  } else if (userChoice.value === UserOptions.JoinExisting) {
    initializationStep.value = InitializationStep.JoinStation;
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
