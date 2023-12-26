<template>
    <a-menu-item v-for="item in props.items" :key="item.key">
        <span><icon-book />{{ item.name }}</span>
    </a-menu-item>
</template>

<script setup lang="ts">
import { IconBook } from "@arco-design/web-vue/es/icon";

const props = defineProps({
    items: {
        type: Object,
        default: [],
    },
});
</script>
