<template>
  <VFooter color="transparent flex-grow-0">
    <VContainer fluid class="px-2">
      <VRow>
        <VCol cols="12" class="d-flex justify-start px-0">
          <a href="https://internetcomputer.org" target="_blank">
            <VImg :src="poweredByBadge" alt="Internet Computer" width="180" />
          </a>
        </VCol>
      </VRow>
    </VContainer>
  </VFooter>
</template>

<script lang="ts" setup>
import { VCol, VContainer, VFooter, VImg, VRow } from 'vuetify/components';

const poweredByBadge = `/images/powered-by-badge.svg`;
</script>
