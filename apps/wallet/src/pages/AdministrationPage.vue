<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="pageTitle" :breadcrumbs="props.breadcrumbs" />
    </template>
    <template #main-body>
      <PageBody>
        <VContainer class="pa-0" fluid>
          <VRow>
            <AuthCheck :privileges="[Privilege.ListProposals]">
              <template #default>
                <VCol cols="12" md="4">
                  <StationInfoCard />
                </VCol>
                <VCol cols="12" md="8">
                  <RecentProposals
                    :title="$t(`app.station_upgrades_card_title`)"
                    :types="[{ ChangeCanister: null }]"
                  >
                    <template #top-actions>
                      <AuthCheck :privileges="[Privilege.ChangeCanister]">
                        <ChangeCanisterActionBtn />
                      </AuthCheck>
                      <VBtn
                        variant="elevated"
                        color="secondary"
                        size="small"
                        :to="{
                          name: Routes.Proposals,
                          query: { group_by: ProposalDomains.System },
                        }"
                      >
                        {{ $t('terms.see_all') }}
                      </VBtn>
                    </template>
                  </RecentProposals>
                </VCol>
              </template>
              <template #unauthorized>
                <VCol cols="12" md="4">
                  <StationInfoCard />
                </VCol>
              </template>
            </AuthCheck>
          </VRow>
        </VContainer>
      </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VCol } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import PageLayout from '~/components/PageLayout.vue';
import ChangeCanisterActionBtn from '~/components/change-canister/ChangeCanisterActionBtn.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import RecentProposals from '~/components/proposals/RecentProposals.vue';
import StationInfoCard from '~/components/settings/StationInfoCard.vue';
import { Routes } from '~/configs/routes.config';
import { PageProps } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import { ProposalDomains } from '~/types/station.types';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });
const i18n = useI18n();
const pageTitle = computed(() => props.title || i18n.t('pages.administration.title'));
</script>
