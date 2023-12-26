<template>
    <a-menu @menuItemClick="onClickMenuItem" @subMenuClick="onClickMenuItem">
        <Notes :items="menuStore.menu.items" />
        <Groups :menu="menuStore.menu.submenus" />
        <a-button type="primary" @click="menuStore.getMenu">Open</a-button>
    </a-menu>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";
import { useMenuStore } from "../../stores/menu";
import Notes from "./Notes.vue";
import Groups from "./Groups.vue";
import { invoke } from "@tauri-apps/api";

const menuStore = useMenuStore();

const onClickMenuItem = async (key: string) => {
    if (await invoke("is_note", { key: key })) {
        menuStore.getNoteContents(key);
        menuStore.setIsNoteEditing(true);
        menuStore.setEditingNoteKey(key);
    } else {
        menuStore.setIsNoteEditing(false);
        menuStore.setEditingNoteKey("");
    }
    Message.info({ content: `You select ${key}`, showIcon: true });
};
</script>
