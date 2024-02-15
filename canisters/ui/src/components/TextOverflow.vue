<template>
  <div class="d-inline" :title="props.text" @copy="handleCopy">
    <span aria-hidden="true">{{ truncatedText }}</span>
    <span class="d-none" tabindex="-1">{{ props.text }}</span>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue';

const props = withDefaults(
  defineProps<{
    text: string;
    maxLength?: number;
    overflowText?: string;
    overflowPosition?: 'start' | 'middle' | 'end';
  }>(),
  {
    maxLength: 18,
    overflowText: '...',
    overflowPosition: 'middle',
  },
);

const truncatedText = computed(() => {
  if (props.text.length <= props.maxLength) {
    return props.text;
  }

  if (props.overflowPosition === 'start') {
    return `${props.overflowText}${props.text.slice(
      props.text.length - props.maxLength,
      props.text.length - props.overflowText.length,
    )}`;
  }

  if (props.overflowPosition === 'end') {
    return `${props.text.slice(0, props.maxLength - props.overflowText.length)}${
      props.overflowText
    }`;
  }

  const overflowLengthStart = Math.ceil(props.overflowText.length / 2);
  const overflowLengthEnd = Math.floor(props.overflowText.length / 2);
  const start = Math.ceil((props.maxLength - 1) / 2) - overflowLengthStart;
  const end = Math.floor((props.maxLength - 1) / 2) - overflowLengthEnd;

  return `${props.text.slice(0, start)}${props.overflowText}${props.text.slice(
    props.text.length - end,
    props.text.length,
  )}`;
});

const handleCopy = (event: ClipboardEvent): void => {
  event.preventDefault();

  if (event.clipboardData) {
    // Set the full text to the clipboard
    event.clipboardData.setData('text/plain', props.text);
  }
};
</script>
