<template>
    <a-menu-item class="menu-item" v-for="item in props.items" :key="item.key">
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

<style scoped>
.menu-item {
    font-size: 16px;
    font-stretch: condensed;
    background: linear-gradient(-45deg, rgb(250, 246, 255), rgb(255, 255, 255));
}
</style>
