<template>
  <VRow>
    <VCol cols="12">
      <VSelect
        v-model="obtainStrategySelected"
        :label="$t('external_canisters.monitor.strategy.label')"
        :readonly="props.readonly"
        :items="strategies"
        :prepend-icon="mdiCog"
        :rules="[requiredRule]"
        name="obtain-strategy"
      />
    </VCol>
  </VRow>
  <template v-if="model.cycleObtainStrategy">
    <template
      v-if="variantIs(model.cycleObtainStrategy, ObtainCyclesStrategyEnum.MintFromNativeToken)"
    >
      <VRow class="mt-0">
        <VCol cols="12">
          <MintFromNativeToken v-model="selectedAccountId" variant="filled"></MintFromNativeToken>
        </VCol>
      </VRow>
    </template>
  </template>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { VCol, VRow } from 'vuetify/components';
import { useI18n } from 'vue-i18n';
import { variantIs } from '~/utils/helper.utils.ts';
import { requiredRule } from '~/utils/form.utils.ts';
import { mdiCog } from '@mdi/js';
import { CanisterMonitorModel } from '~/components/external-canisters/external-canisters.types.ts';
import MintFromNativeToken from '~/components/settings/obtain-cycles/MintFromNativeToken.vue';
import { ObtainCyclesStrategyEnum } from '~/components/external-canisters/monitor/monitor.types.ts';

const props = withDefaults(
  defineProps<{
    modelValue: CanisterMonitorModel;
    readonly?: boolean;
    display?: {
      canisterId: boolean;
    };
  }>(),
  {
    readonly: false,
    display: () => ({
      canisterId: true,
    }),
  },
);

const i18n = useI18n();

const emit = defineEmits<{
  (event: 'update:modelValue', payload: CanisterMonitorModel): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const selectedAccountId = computed({
  get: () => {
    if (
      model.value.cycleObtainStrategy &&
      ObtainCyclesStrategyEnum.MintFromNativeToken in model.value.cycleObtainStrategy &&
      model.value.cycleObtainStrategy.MintFromNativeToken.account_id
    ) {
      return model.value.cycleObtainStrategy.MintFromNativeToken.account_id;
    }

    return null;
  },
  set: value => {
    if (value) {
      model.value.cycleObtainStrategy = {
        MintFromNativeToken: {
          account_id: value,
        },
      };
    } else {
      model.value.cycleObtainStrategy = undefined;
    }
  },
});

const obtainStrategySelected = computed({
  get: () => {
    if (!model.value.cycleObtainStrategy) {
      return ObtainCyclesStrategyEnum.StationDefault;
    }

    return ObtainCyclesStrategyEnum.MintFromNativeToken;
  },
  set: value => {
    switch (value) {
      case ObtainCyclesStrategyEnum.MintFromNativeToken:
        model.value.cycleObtainStrategy = {
          MintFromNativeToken: {
            account_id: '',
          },
        };
        break;
      case ObtainCyclesStrategyEnum.StationDefault:
        model.value.cycleObtainStrategy = undefined;
        break;
    }
  },
});

const strategyKeys = ref<ObtainCyclesStrategyEnum[]>([
  ObtainCyclesStrategyEnum.StationDefault,
  // TODO we need to ensure teams cannot drain accounts
  // that means each monitoring request should be approved by an owner of the account
  // ObtainCyclesStrategyEnum.MintFromNativeToken,
]);

const strategies = computed<
  {
    title: string;
    value: string;
  }[]
>(() =>
  strategyKeys.value.map(key => ({
    title: i18n.t(`external_canisters.obtain.strategy.${key}`),
    value: key,
  })),
);
</script>
