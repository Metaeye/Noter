<template>
    <a-menu-item v-for="note in props.notes" :key="note.key">
        <span><icon-book />{{ note.title }}</span>
    </a-menu-item>
</template>

<script setup lang="ts">
import { IconBook } from "@arco-design/web-vue/es/icon";

const props = defineProps({
    notes: {
        type: Object,
        default: [],
    },
});
</script>
