<template>
  <Teleport v-if="mounted" to="#policies-actions">
    <AuthCheck :privileges="[Privilege.AddNamedRule]">
      <NamedRuleDialogBtn :text="$t('pages.approval_policy.btn_new_entry')" />
    </AuthCheck>
  </Teleport>

  <DataLoader
    v-slot="{ loading }"
    v-model:force-reload="forceReload"
    :disable-refresh="disableRefresh"
    :load="fetchList"
    :refresh-interval-ms="5000"
    @loaded="
      result => {
        namedRules = result.named_rules;
        privileges = result.privileges;
      }
    "
  >
    <VDataTable
      class="elevation-2 rounded"
      :loading="loading"
      :headers="headers"
      :items="namedRules"
      :items-per-page="-1"
      :hover="true"
    >
      <template #bottom>
        <!--this hides the footer as pagination is not required-->
      </template>
      <template #item.name="{ item: namedRule }">
        <div class="text-body-2">{{ namedRule.name }}</div>
        <div v-if="namedRule.description[0]" class="text-caption text-grey-lighten-1">
          {{ namedRule.description[0] }}
        </div>
      </template>
      <template #item.linked_policies="{ item: namedRule }">
        <div v-if="linkedPolicies">
          {{ linkedPolicies[namedRule.id] || '-' }}
        </div>
      </template>
      <template #item.rule="{ item: namedRule }">
        <RuleSummary :rule="namedRule.rule" />
      </template>
      <template #item.actions="{ item: namedRule }">
        <div class="d-flex justify-end">
          <ActionBtn
            v-if="hasDeletePrivilege(namedRule.id)"
            v-model="namedRule.id"
            :icon="mdiTrashCanOutline"
            :submit="id => station.service.removeNamedRule(id)"
            :disabled="(linkedPolicies?.[namedRule.id] ?? 0) > 0"
            @failed="useOnFailedOperation"
            @submitted="useOnSuccessfulOperation"
          />
          <NamedRuleDialogBtn
            :icon="!hasEditPrivilege(namedRule.id) ? mdiEye : mdiPencil"
            :named-rule-id="namedRule.id"
            :readonly="!hasEditPrivilege(namedRule.id)"
            variant="flat"
            color="default"
            size="small"
            @opened="disableRefresh = $event"
          />
        </div>
      </template>
    </VDataTable>
  </DataLoader>
  <VPagination
    v-model="pagination.selectedPage"
    class="mt-2"
    :length="pagination.totalPages"
    rounded
    density="comfortable"
    @update:model-value="triggerSearch"
  />
</template>

<script lang="ts" setup>
import { mdiEye, mdiPencil, mdiTrashCanOutline } from '@mdi/js';
import { computed, ComputedRef, onMounted, onUnmounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useDisplay } from 'vuetify';
import { VDataTable, VPagination } from 'vuetify/components';
import AuthCheck from '~/components/AuthCheck.vue';
import DataLoader from '~/components/DataLoader.vue';
import ActionBtn from '~/components/buttons/ActionBtn.vue';
import NamedRuleDialogBtn from '~/components/request-policies/NamedRuleDialogBtn.vue';
import RuleSummary from '~/components/request-policies/rule/RuleSummary.vue';
import { useFetchList, usePagination } from '~/composables/lists.composable';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import {
  ListRequestPoliciesResult,
  NamedRule,
  NamedRuleCallerPrivileges,
  RequestPolicy,
  RequestPolicyRule,
  UUID,
} from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { Privilege } from '~/types/auth.types';
import { ExtractOk } from '~/types/helper.types';
import { throttle, variantIs } from '~/utils/helper.utils';

const station = useStationStore();
const i18n = useI18n();
const namedRules = ref<NamedRule[]>([]);
const privileges = ref<NamedRuleCallerPrivileges[]>([]);
const disableRefresh = ref(false);
const forceReload = ref(false);
const { xs } = useDisplay();
const pagination = usePagination();
const triggerSearch = throttle(() => (forceReload.value = true), 500);
const allPolicies = ref<RequestPolicy[] | null>(null);
const mounted = ref(false);

const headers = computed(() => {
  return [
    { title: i18n.t('terms.name'), key: 'name', sortable: false },

    ...(xs.value
      ? []
      : [
          { title: i18n.t('terms.rule'), key: 'rule', sortable: false },
          {
            title: i18n.t('pages.approval_policy.linked_policies'),
            key: 'linked_policies',
            sortable: false,
          },
        ]),
    { title: '', key: 'actions', sortable: false },
  ];
});

const hasEditPrivilege = (id: UUID): boolean => {
  const privilege = privileges.value.find(p => p.id === id);
  return !!privilege?.can_edit;
};

const hasDeletePrivilege = (id: UUID): boolean => {
  const privilege = privileges.value.find(p => p.id === id);
  return !!privilege?.can_delete;
};

let useVerifiedCall = false;

function ruleHasNamedRule(rule: RequestPolicyRule, namedRuleId: UUID): boolean {
  if (variantIs(rule, 'NamedRule')) {
    return rule.NamedRule === namedRuleId;
  } else if (variantIs(rule, 'AllOf')) {
    return rule.AllOf.some(r => ruleHasNamedRule(r, namedRuleId));
  } else if (variantIs(rule, 'AnyOf')) {
    return rule.AnyOf.some(r => ruleHasNamedRule(r, namedRuleId));
  } else if (variantIs(rule, 'Not')) {
    return ruleHasNamedRule(rule.Not, namedRuleId);
  }
  return false;
}

function namedRuleIsUsedInNamedRules(
  namedRuleId: UUID,
  named_rule_rules: RequestPolicyRule[],
): boolean {
  return named_rule_rules.some(nr => ruleHasNamedRule(nr, namedRuleId));
}

const fetchList = useFetchList(
  (offset, limit) => {
    const results = station.service.listNamedRules(
      {
        offset,
        limit,
      },
      useVerifiedCall,
    );

    useVerifiedCall = true;

    return results;
  },
  {
    pagination,
    getTotal: res => Number(res.total),
  },
);

onMounted(async () => {
  mounted.value = true;

  const fetchedPolicies: RequestPolicy[] = [];
  let componentUnmounted = false;

  onUnmounted(() => {
    componentUnmounted = true;
  });

  let offset = 0;
  let result: ExtractOk<ListRequestPoliciesResult>;
  do {
    result = await station.service.listRequestPolicies({
      limit: 100,
      offset: offset,
    });
    fetchedPolicies.push(...result.policies);
    if (result.next_offset.length === 0 || componentUnmounted) {
      break;
    }

    offset = Number(result.next_offset);
  } while (result.total > fetchedPolicies.length);

  if (!componentUnmounted) {
    allPolicies.value = fetchedPolicies;
  }
});

const linkedPolicies: ComputedRef<Record<UUID, number> | null> = computed(() => {
  if (!allPolicies.value || !namedRules.value) {
    return null;
  }

  const allNamedRuleRules = namedRules.value.flatMap(nr => nr.rule);

  const linkedPolicies: Record<UUID, number> = {};
  for (const namedRule of namedRules.value) {
    if (namedRuleIsUsedInNamedRules(namedRule.id, allNamedRuleRules)) {
      linkedPolicies[namedRule.id] = (linkedPolicies[namedRule.id] || 0) + 1;
    }

    for (const policy of allPolicies.value) {
      if (ruleHasNamedRule(policy.rule, namedRule.id)) {
        linkedPolicies[namedRule.id] = (linkedPolicies[namedRule.id] || 0) + 1;
      }
    }
  }
  return linkedPolicies;
});
</script>
