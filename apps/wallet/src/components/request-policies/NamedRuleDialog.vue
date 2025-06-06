<template>
  <VDialog
    v-model="openModel"
    :persistent="loading || saving"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth.value"
  >
    <DataLoader
      v-slot="{ data }"
      :load="loadNamedRule"
      @loading="loading = $event"
      @loaded="namedRule = $event"
    >
      <VCard>
        <VToolbar color="background">
          <VToolbarTitle>{{ $t('pages.approval_rules.dialog.title') }}</VToolbarTitle>
          <VBtn :disabled="loading || saving" :icon="mdiClose" @click="openModel = false" />
        </VToolbar>
        <VCardText v-if="loading" class="py-8">
          <LoadingMessage />
        </VCardText>
        <VCardText v-else>
          <NamedRuleForm
            v-if="data"
            v-model="namedRule"
            :mode="props.readonly.value ? 'view' : 'edit'"
            @submit="save"
            @valid="valid = $event"
          />
        </VCardText>
        <VDivider />
        <VCardActions class="pa-3">
          <VSpacer />
          <VBtn
            v-if="!props.readonly.value"
            color="primary"
            variant="elevated"
            :disabled="!canSave"
            :loading="saving"
            data-test-id="save-named-rule"
            @click="save"
          >
            {{ $t('terms.save') }}
          </VBtn>
        </VCardActions>
      </VCard>
    </DataLoader>
  </VDialog>
</template>
<script lang="ts" setup>
import { mdiClose } from '@mdi/js';
import { computed, Ref, ref, toRefs } from 'vue';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VDivider,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import DataLoader from '~/components/DataLoader.vue';
import LoadingMessage from '~/components/LoadingMessage.vue';
import {
  useOnFailedOperation,
  useOnSuccessfulOperation,
} from '~/composables/notifications.composable';
import logger from '~/core/logger.core';
import { NamedRule, UUID } from '~/generated/station/station.did';
import { useStationStore } from '~/stores/station.store';
import { assertAndReturn } from '~/utils/helper.utils';
import NamedRuleForm from './NamedRuleForm.vue';

const input = withDefaults(
  defineProps<{
    namedRuleId?: UUID;
    open?: boolean;
    dialogMaxWidth?: number;
    readonly?: boolean;
  }>(),
  {
    namedRuleId: undefined,
    open: false,
    dialogMaxWidth: 800,
    readonly: false,
  },
);

const emit = defineEmits<{
  (event: 'update:open', payload: boolean): void;
}>();

const props = toRefs(input);
const valid = ref(false);
const loading = ref(false);
const saving = ref(false);
const namedRule: Ref<Partial<NamedRule>> = ref({});
const openModel = computed({
  get: () => props.open.value,
  set: value => emit('update:open', value),
});

const station = useStationStore();

const loadNamedRule = async (): Promise<Partial<NamedRule>> => {
  if (props.namedRuleId.value === undefined) {
    const createModel: Partial<NamedRule> = {
      rule: undefined,
      name: '',
      description: [],
    };

    return createModel;
  }
  const result = await station.service.getNamedRule(props.namedRuleId.value);
  return result.named_rule;
};

const canSave = computed(() => {
  return valid.value && !loading.value && !!namedRule.value?.rule;
});

const save = async (): Promise<void> => {
  if (!canSave.value) {
    return;
  }

  try {
    saving.value = true;
    if (namedRule.value.id) {
      const request = await station.service.editNamedRule({
        named_rule_id: namedRule.value.id,
        name: [assertAndReturn(namedRule.value.name)],
        description: namedRule.value.description ? [namedRule.value.description] : [['']],
        rule: [assertAndReturn(namedRule.value.rule)],
      });

      useOnSuccessfulOperation(request);

      openModel.value = false;
      return;
    }

    const request = await station.service.addNamedRule({
      name: assertAndReturn(namedRule.value.name),
      description: assertAndReturn(namedRule.value.description),
      rule: assertAndReturn(namedRule.value.rule),
    });

    useOnSuccessfulOperation(request);

    openModel.value = false;
  } catch (error) {
    logger.error(`Failed to save named rule ${error}`);

    useOnFailedOperation();
  } finally {
    saving.value = false;
  }
};
</script>
