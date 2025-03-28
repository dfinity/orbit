<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :breadcrumbs="props.breadcrumbs">
        <template #actions>
          <div id="policies-actions"></div>
        </template>
        <template #title>
          <VSlideGroup v-model="tab">
            <VSlideGroupItem v-for="tabInfo in tabs" :key="tabInfo.name">
              <VBtn
                size="small"
                variant="text"
                :active="tab === tabInfo.name"
                @click="tab = tabInfo.name"
              >
                {{ $t(`navigation.${tabInfo.name}`) }}
              </VBtn>
            </VSlideGroupItem>
          </VSlideGroup>
        </template>
      </PageHeader>
    </template>

    <template #main-body>
      <PageBody>
        <AuthCheck :privileges="[Privilege.ListRequests, Privilege.ListNamedRules]">
          <RecentRequests
            class="mb-4"
            :see-all-link="{
              name: Routes.Requests,
              query: { group_by: RequestDomains.System },
            }"
            :types="[
              { AddNamedRule: null },
              { EditNamedRule: null },
              { RemoveNamedRule: null },
              { AddRequestPolicy: null },
              { EditRequestPolicy: null },
              { RemoveRequestPolicy: null },
            ]"
            hide-not-found
          />
        </AuthCheck>
        <component :is="tabs.find(t => t.name === tab)?.component" v-if="tab" />
      </PageBody>
    </template>
  </PageLayout>
</template>

<script setup lang="ts">
import { PageProps } from '~/types/app.types';
import { Privilege } from '~/types/auth.types';
import PageLayout from '~/components/PageLayout.vue';
import PageHeader from '~/components/layouts/PageHeader.vue';
import AuthCheck from '~/components/AuthCheck.vue';
import PageBody from '~/components/layouts/PageBody.vue';
import RequestPoliciesTab from '~/components/policies/RequestPoliciesTab.vue';
import ApprovalRulesTab from '~/components/policies/ApprovalRulesTab.vue';
import { Routes } from '~/configs/routes.config';
import { RequestDomains } from '~/types/station.types';
import RecentRequests from '~/components/requests/RecentRequests.vue';
import { type Component, ref } from 'vue';
import { hasRequiredPrivilege } from '~/utils/auth.utils';

const props = withDefaults(defineProps<PageProps>(), { title: undefined, breadcrumbs: () => [] });

enum Tabs {
  RequestPolicies = 'request_policies',
  ApprovalRules = 'approval_rules',
}
const tabs: { name: Tabs; component: Component }[] = [];

if (hasRequiredPrivilege({ anyOf: [Privilege.ListNamedRules] })) {
  tabs.push({ name: Tabs.ApprovalRules, component: ApprovalRulesTab });
}

if (hasRequiredPrivilege({ anyOf: [Privilege.ListRequestPolicies] })) {
  tabs.push({ name: Tabs.RequestPolicies, component: RequestPoliciesTab });
}

const tab = ref<Tabs | undefined>(tabs[0]?.name);
</script>
