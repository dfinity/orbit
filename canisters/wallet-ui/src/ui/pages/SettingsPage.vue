<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pt-16 pb-13 pl-8 pr-8" fluid>
        <VRow>
          <VCol cols="12">
            <h1 class="text-h4">{{ $t('terms.settings') }}</h1>
            <p>{{ $t('settings.subtitle') }}</p>
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VRow>
        <VCol cols="12">
          <VCard color="background" :loading="settings.isLoading">
            <VTabs v-model="tab" center-active class="px-8">
              <VTab
                value="settingsForm"
                :loading="settings.isLoading"
                class="settings_tab__item"
                size="small"
              >
                {{ $t('terms.control_panel') }}
              </VTab>
            </VTabs>
            <VCardText>
              <VWindow v-model="tab">
                <VWindowItem value="settingsForm">
                  <SettingsForm />
                </VWindowItem>
              </VWindow>
            </VCardText>
          </VCard>
        </VCol>
      </VRow>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import SettingsForm from '~/ui/components/SettingsForm.vue';
import { useSettingsFormStore } from '~/ui/stores';

const tab = ref<'settingsForm'>('settingsForm');
const settings = useSettingsFormStore();
</script>

<style scoped lang="scss">
.settings_tab {
  &__item {
    background: rgb(var(--ds-surface));
  }

  &__item.v-slide-group-item--active {
    font-weight: 600;
  }
}
</style>
