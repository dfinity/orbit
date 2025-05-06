<template>
  <div class="d-flex ga-4">
    <div class="text-medium-emphasis"><VIcon :icon="mdiCodeArray" /></div>
    <div class="d-flex flex-column ga-0 flex-grow-1">
      <div class="d-flex flex-nowrap">
        <VBtnToggle v-model="selectedParseFormat" rounded="0" group density="compact">
          <VBtn
            v-for="(format, idx) in availableParseFormats"
            :key="idx"
            :value="format.value"
            variant="tonal"
            size="small"
          >
            {{ format.text }}
          </VBtn>
        </VBtnToggle>
      </div>
      <VTextarea
        v-model="argument"
        v-model:focused="argumentInputFocused"
        :label="label"
        :readonly="props.readonly"
        :density="props.density"
        :variant="props.variant"
        :rules="[...(props.required ? [requiredRule] : []), parseArgumentRule]"
        :rows="3"
        class="mt-0"
        v-bind="$attrs"
      />
    </div>
  </div>
</template>
<script setup lang="ts">
import { mdiCodeArray } from '@mdi/js';
import { computed, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { VBtn, VBtnToggle, VIcon, VTextarea } from 'vuetify/components';
import { SelectItem } from '~/types/helper.types';
import { hexStringToArrayBuffer } from '~/utils/crypto.utils';
import { encode } from '~/utils/didc.utils';
import { isHexRule, requiredRule } from '~/utils/form.utils';
import { assertAndReturn } from '~/utils/helper.utils';

const props = withDefaults(
  defineProps<{
    modelValue?: Uint8Array;
    /**
     * Enables the argument to be written in Candid textual representation.
     */
    candid?: {
      idl: string;
      method?: string;
    };
    readonly?: boolean;
    required?: boolean;
    label?: string;
    density?: 'comfortable' | 'compact' | 'default';
    variant?: 'filled' | 'outlined' | 'plain' | 'solo' | 'underlined';
  }>(),
  {
    modelValue: undefined,
    readonly: false,
    required: false,
    label: undefined,
    candid: undefined,
    density: 'comfortable',
    variant: 'filled',
  },
);

const emit = defineEmits<{
  (event: 'update:modelValue', payload?: Uint8Array): void;
}>();

const model = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value),
});

const i18n = useI18n();
const label = computed(() => {
  if (props.label) {
    return props.label;
  }

  return props.required
    ? i18n.t('external_canisters.wasm_args')
    : i18n.t('external_canisters.wasm_args_optional');
});

const argument = ref<string>();
const selectedParseFormat = ref<string>(props.candid ? 'candid' : 'hex');
const availableParseFormats = computed<SelectItem[]>(() => {
  const items: SelectItem[] = [];

  if (props.candid) {
    items.push({ text: i18n.t('external_canisters.wasm_args_formats.candid'), value: 'candid' });
  }

  items.push({ text: i18n.t('external_canisters.wasm_args_formats.hex'), value: 'hex' });

  return items;
});

const argumentInputFocused = ref(false);
const parseArgumentRule = async (value: unknown): Promise<string | boolean> => {
  try {
    if (argumentInputFocused.value) {
      // Skip validation if the input is focused to avoid showing errors and expensive parsing while typing.
      // The validation will be triggered on blur.
      return true;
    }

    const hasValue = !!value;
    if (!hasValue) {
      model.value = undefined;
      // this rule only applies if there is a value
      return true;
    }

    if (typeof value !== 'string') {
      throw new Error(i18n.t('external_canisters.wasm_args_invalid_format'));
    }

    let rawArgument = value.trim();
    let parsedArgument: Uint8Array;

    switch (selectedParseFormat.value) {
      case 'hex': {
        const hexValidation = isHexRule(rawArgument);
        if (hexValidation !== true) {
          throw new Error(hexValidation);
        }

        parsedArgument = new Uint8Array(hexStringToArrayBuffer(rawArgument));
        break;
      }
      case 'candid': {
        const candid = assertAndReturn(props.candid, 'Candid definition is expected');
        const hexString = encode({
          idl: candid.idl,
          withType: candid.method
            ? {
                kind: 'methodParams',
                name: candid.method,
              }
            : undefined,
          input: rawArgument,
          targetFormat: 'hex',
        });

        parsedArgument = new Uint8Array(hexStringToArrayBuffer(hexString));
        break;
      }
      default:
        throw new Error('Not implemented');
    }

    model.value = parsedArgument;
    return true;
  } catch (e) {
    // Resets the model if the argument in the input is invalid
    model.value = undefined;

    return e instanceof Error ? e.message : `${e}`;
  }
};

watch(selectedParseFormat, (format, oldFormat) => {
  if (format !== oldFormat && oldFormat !== undefined) {
    argument.value = '';
    model.value = undefined;
  }
});
</script>
