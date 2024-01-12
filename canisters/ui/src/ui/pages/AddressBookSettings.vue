<template>
  <PageLayout>
    <template #main-header>
      <VContainer class="pa-8" fluid>
        <VRow>
          <VCol cols="12">
            <h1 class="text-h4">Address Book - Settings</h1>
          </VCol>
        </VRow>
      </VContainer>
    </template>
    <template #main-body>
      <VContainer class="pl-8 pr-8" fluid>
        <VRow>
          <VCol cols="12"> page content </VCol>
        </VRow>
      </VContainer>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import PageLayout from '~/ui/components/PageLayout.vue';
</script>
