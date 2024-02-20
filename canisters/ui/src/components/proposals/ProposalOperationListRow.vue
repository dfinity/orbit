<template>
  <div class="d-flex flex-no-wrap align-start flex-row ga-2">
    <slot name="default">
      <div class="font-weight-bold">
        <slot name="name"></slot>
      </div>
      <div class="text-medium-emphasis">
        <slot name="content"></slot>
      </div>
    </slot>
  </div>
</template>
