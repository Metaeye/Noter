<template>
    <a-sub-menu v-for="group in props.groups" :key="group.key">
        <template #title>
            <span title="Title"><icon-folder />{{ group.title }}</span>
        </template>
        <Notes :notes="group.notes" />
        <Groups :groups="group.groups" />
    </a-sub-menu>
</template>

<script setup lang="ts">
import { IconFolder } from "@arco-design/web-vue/es/icon";
import Notes from "./Notes.vue";

const props = defineProps({
    groups: {
        type: Object,
        default: [],
    },
});
</script>
