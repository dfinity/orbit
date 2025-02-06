<template>
  <VFooter color="transparent">
    <VContainer fluid :class="{ 'px-16': !app.isMobile }">
      <VRow class="border-bottom flex-column-reverse flex-md-row">
        <VCol cols="12" md="6" class="d-flex align-center flex-row ga-6 px-0">
          <div class="d-flex ga-3 align-center">
            <a href="https://github.com/dfinity/orbit" target="_blank">
              <GithubMark height="28" />
            </a>
            <VImg :src="betaImage" width="40" />
          </div>
          <div class="flex-grow-1 text-body-2 text-md-left text-right">
            <!-- TODO: Add license link -->
            <a href="#" target="_blank" class="text-decoration-none text-uppercase">
              {{ $t('terms.license') }}
            </a>
          </div>
        </VCol>
        <VCol cols="12" md="6" class="d-flex align-center justify-md-end px-0 py-0">
          <a href="https://internetcomputer.org" target="_blank">
            <PoweredByImage width="180" alt="Internet Computer" draggable="false" />
          </a>
        </VCol>
      </VRow>
    </VContainer>
  </VFooter>
</template>

<script lang="ts" setup>
import { VCol, VContainer, VFooter, VRow } from 'vuetify/components';
import PoweredByImage from '~/components/ui/svg/PoweredByImage.vue';
import { useAppStore } from '~/stores/app.store';
import betaImage from '~assets/images/beta.png';
import GithubMark from '~/components/ui/svg/GithubMark.vue';

const app = useAppStore();
</script>
