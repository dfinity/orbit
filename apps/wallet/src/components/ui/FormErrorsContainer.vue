<template>
  <VAlert v-if="errors.length" type="error" variant="tonal" density="compact" class="ma-4">
    <VListItem v-for="error in errors" :key="error.field">
      <VListItemTitle>{{ error.field }}</VListItemTitle>
      <VListItemSubtitle>
        <template v-for="(message, idx) in error.errorMessages" :key="idx">
          {{ message }}<br />
        </template>
      </VListItemSubtitle>
    </VListItem>
  </VAlert>
</template>
<script lang="ts" setup>
import { toRefs } from 'vue';
import { FieldWithError } from '~/composables/forms.composable';

const props = withDefaults(
  defineProps<{
    errors?: FieldWithError[];
  }>(),
  {
    errors: () => [],
  },
);

const { errors } = toRefs(props);
</script>
