<template>
  <VBtn
    v-bind="$attrs"
    :data-test-id="props.dataTestId"
    :size="props.size"
    :icon="!props.text"
    :variant="props.variant"
    :rounded="props.rounded"
    :color="props.color"
    :class="props.class"
    @click="open = true"
  >
    <VIcon v-if="props.icon && typeof props.icon === 'string'" :icon="props.icon" />
    <span v-if="props.text">{{ props.text }}</span>
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
      <VToolbar dark :color="props.dialogToolbarColor">
        <VToolbarTitle
          :data-test-id="props.dataTestId ? `${props.dataTestId}-dialog-title` : undefined"
        >
          {{ props.title }}
        </VToolbarTitle>
        <VBtn :disabled="loading" :icon="mdiClose" dark @click="close" />
      </VToolbar>
      <VCardText>
        <slot name="default" :model="{ value: intervalValue }" :loading="loading" :submit="submit">
          <p>{{ props.content }}</p>
        </slot>
      </VCardText>
      <VCardActions class="px-6 py-3">
        <slot
          name="actions"
          :submit="submit"
          :close="close"
          :loading="loading"
          :model="{ value: intervalValue }"
        >
          <VSpacer />
          <VBtn variant="text" @click="close">{{ props.cancelText }}</VBtn>
          <VBtn :loading="loading" color="primary" variant="flat" @click="submit">
            {{ props.confirmText }}
          </VBtn>
        </slot>
      </VCardActions>
    </VCard>
  </VDialog>
</template>

<script lang="ts" setup generic="T, M">
import { mdiClose } from '@mdi/js';
import { Ref, computed, ref, watch } from 'vue';
import { logger, wait } from '~/core';
import { i18n } from '~/ui/modules/i18n';

const props = withDefaults(
  defineProps<{
    text?: string;
    icon?: string | boolean;
    size?: 'x-small' | 'small' | 'default' | 'large' | 'x-large';
    variant?: 'text' | 'flat' | 'elevated' | 'tonal' | 'outlined' | 'plain';
    color?: string;
    title?: string;
    content?: string;
    cancelText?: string;
    confirmText?: string;
    class?: string;
    rounded?: boolean;
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
    size: 'small',
    variant: 'flat',
    color: 'default',
    title: i18n.global.t('app.dialog_confirmation_title'),
    content: i18n.global.t('app.dialog_confirmation_question'),
    cancelText: i18n.global.t('terms.cancel'),
    confirmText: i18n.global.t('terms.confirm'),
    class: undefined,
    rounded: false,
    dialogMaxWidth: 800,
    dialogToolbarColor: 'surface',
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
