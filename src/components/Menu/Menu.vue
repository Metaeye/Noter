<template>
    <a-menu @menuItemClick="onClickMenuItem" @subMenuClick="onClickMenuItem">
        <Notes :items="menuStore.root.items" />
        <Groups :menu="menuStore.root.submenus" />
        <a-button type="primary" @click="menuStore.get_menu">Open</a-button>
    </a-menu>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";
import { useMenuStore } from "../../stores/menu";
import Notes from "./Notes.vue";
import Groups from "./Groups.vue";

const menuStore = useMenuStore();

const onClickMenuItem = (key: string) => {
    // const value = menuStore.find_in_root(key);
    // menuStore.curNote = Object.keys(value).includes("contents") ? value : undefined;
    Message.info({ content: `You select ${key}`, showIcon: true });
};
</script>
