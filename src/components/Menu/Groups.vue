<template>
    <a-sub-menu v-for="submenu in props.menu" :key="submenu.key">
        <template #title>
            <span title="Title"><icon-folder />{{ submenu.title }}</span>
        </template>
        <Notes :items="submenu.items" />
        <Groups :menu="submenu.submenus" />
    </a-sub-menu>
</template>

<script setup lang="ts">
import { IconFolder } from "@arco-design/web-vue/es/icon";
import Notes from "./Notes.vue";

const props = defineProps({
    menu: {
        type: Object,
        default: [],
    },
});
</script>
