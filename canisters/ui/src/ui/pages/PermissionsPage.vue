<template>
  <PageLayout>
    <template #main-header>
      <PageHeader :title="props.title" :breadcrumbs="props.breadcrumbs">
        <template #actions>
          <AuthCheck :privileges="[Privilege.AddAccessPolicy]">
            <!--todo: add create access policy action btn-->
            <VBtn color="primary-variant" variant="flat" disabled>
              {{ $t('proposals.types.addaccesspolicy.title') }}
            </VBtn>
          </AuthCheck>
        </template>
      </PageHeader>
    </template>

    <template #main-body>
      <PageBody> todo </PageBody>
    </template>
  </PageLayout>
</template>

<script lang="ts" setup>
import { Privilege } from '~/types';
import AuthCheck from '~/ui/components/AuthCheck.vue';
import PageLayout from '~/ui/components/PageLayout.vue';
import PageBody from '~/ui/components/layouts/PageBody.vue';
import PageHeader from '~/ui/components/layouts/PageHeader.vue';
import { i18n } from '~/ui/modules';
import { BreadCrumbItem } from '~/ui/types/navigation';

const props = withDefaults(
  defineProps<{
    title?: string;
    breadcrumbs?: BreadCrumbItem[];
  }>(),
  {
    title: i18n.global.t('pages.permissions.title'),
    breadcrumbs: () => [],
  },
);
</script>
