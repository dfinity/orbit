<template>
  <VBtn
    v-bind="$attrs"
    :data-test-id="props.dataTestId"
    :size="props.size"
    :icon="!props.text"
    :variant="props.variant"
    :density="props.density"
    :rounded="props.rounded"
    :color="props.color"
    :disabled="props.disabled"
    @click="open = true"
  >
    <VIcon
      v-if="props.icon && typeof props.icon === 'string'"
      :icon="props.icon"
      :class="{
        'mr-1': props.text,
      }"
    />
    <span v-if="props.text">{{ props.text }}</span>
    <VIcon v-if="props.appendIcon" :icon="props.appendIcon" class="ml-1" />
  </VBtn>

  <VDialog
    v-model="open"
    :data-test-id="props.dataTestId ? `${props.dataTestId}-dialog` : undefined"
    :persistent="loading"
    transition="dialog-bottom-transition"
    scrollable
    :max-width="props.dialogMaxWidth"
  >
    <VCard :loading="loading">
      <VToolbar :color="props.dialogToolbarColor">
        <VToolbarTitle
          :data-test-id="props.dataTestId ? `${props.dataTestId}-dialog-title` : undefined"
        >
          {{ translations.title }}
        </VToolbarTitle>
        <VBtn :disabled="loading" :icon="mdiClose" @click="close" />
      </VToolbar>
      <VCardText>
        <slot name="default" :model="{ value: intervalValue }" :loading="loading" :submit="submit">
          <p>{{ translations.content }}</p>
        </slot>
      </VCardText>
      <VDivider />
      <VCardActions class="pa-3">
        <slot
          name="actions"
          :submit="submit"
          :close="close"
          :loading="loading"
          :model="{ value: intervalValue }"
        >
          <VSpacer />
          <VBtn variant="outlined" @click="close">{{ translations.cancelText }}</VBtn>
          <VBtn
            :loading="loading"
            color="primary"
            variant="elevated"
            data-test-id="action-btn-default-submit-btn"
            @click="submit"
          >
            {{ translations.confirmText }}
          </VBtn>
        </slot>
      </VCardActions>
    </VCard>
  </VDialog>
</template>

<script lang="ts" setup generic="T, M">
import { mdiClose } from '@mdi/js';
import { Ref, computed, ref, watch } from 'vue';
import { logger } from '~/core/logger.core';
import { wait } from '~/utils/helper.utils';
import {
  VBtn,
  VCard,
  VCardActions,
  VCardText,
  VDialog,
  VDivider,
  VIcon,
  VSpacer,
  VToolbar,
  VToolbarTitle,
} from 'vuetify/components';
import { useI18n } from 'vue-i18n';

const props = withDefaults(
  defineProps<{
    text?: string;
    icon?: string | boolean;
    appendIcon?: string;
    size?: 'x-small' | 'small' | 'default' | 'large' | 'x-large';
    variant?: 'text' | 'flat' | 'elevated' | 'tonal' | 'outlined' | 'plain';
    density?: 'comfortable' | 'default' | 'compact';
    color?: string;
    title?: string;
    content?: string;
    cancelText?: string;
    confirmText?: string;
    rounded?: boolean;
    disabled?: boolean;
    modelValue?: M;
    dialogMaxWidth?: string | number;
    dialogToolbarColor?: string;
    submit?: (model: M) => Promise<T> | T;
    confirmCloseDelayMs?: number;
    clone?: (model: M) => M;
    dataTestId?: string;
  }>(),
  {
    text: undefined,
    icon: false,
    appendIcon: undefined,
    size: 'small',
    variant: 'flat',
    color: 'default',
    density: undefined,
    title: undefined,
    content: undefined,
    cancelText: undefined,
    confirmText: undefined,
    rounded: false,
    disabled: false,
    dialogMaxWidth: 800,
    dialogToolbarColor: 'background',
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    modelValue: null as any,
    submit: undefined,
    confirmCloseDelayMs: 0,
    dataTestId: undefined,
    clone: (model: M) => {
      const cloned = JSON.parse(JSON.stringify({ modelValue: model }));

      return cloned.modelValue;
    },
  },
);

const emit = defineEmits<{
  (event: 'failed', payload: unknown): void;
  (event: 'submitted', payload?: T): void;
  (event: 'closed'): void;
  (event: 'opened'): void;
  (event: 'update:modelValue', payload: M): void;
}>();

const i18n = useI18n();
const translations = computed(() => ({
  title: props.title || i18n.t('app.dialog_confirmation_title'),
  content: props.content || i18n.t('app.dialog_confirmation_question'),
  cancelText: props.cancelText || i18n.t('terms.cancel'),
  confirmText: props.confirmText || i18n.t('terms.confirm'),
}));

// Generics infer the type in a wrong way with refs, so we need to cast it to the correct type
// see: https://github.com/vuejs/core/issues/2136#issuecomment-908269949
const intervalValue = ref<M>(props.modelValue as M) as Ref<M>;

const open = ref<boolean>(false);
const loading = ref<boolean>(false);
const modelValue = computed({
  get: () => props.modelValue,
  set: value => {
    if (value === undefined) {
      logger.warn('unexpected undefined modelValue');
      return;
    }

    emit('update:modelValue', value);
  },
});

const setInternalValue = (value: M | undefined): void => {
  if (value === undefined) {
    return;
  }

  intervalValue.value = props.clone(value);
};

watch(
  () => modelValue.value,
  value => {
    if (open.value || loading.value || value === undefined) {
      // when the dialog is open we don't want to update the intervalValue
      // since that would overwrite the user changes.
      return;
    }

    setInternalValue(value);
  },
  { deep: true, immediate: true },
);

watch(
  () => open.value,
  isOpen => {
    if (!isOpen && !loading.value) {
      setInternalValue(props.modelValue);
      emit('closed');
      return;
    }

    if (isOpen) {
      emit('opened');
    }
  },
);

const close = (): void => {
  open.value = false;
};

const submit = async (): Promise<void> => {
  try {
    loading.value = true;

    let result: T | undefined;
    if (props.submit) {
      result = await props.submit(intervalValue.value);
    }

    emit('submitted', result);

    await wait(props.confirmCloseDelayMs);
    close();
  } catch (error) {
    logger.error(`Error while submitting: ${error}`);

    emit('failed', error);
  } finally {
    loading.value = false;
  }
};
</script>
