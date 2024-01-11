<template>
    <a-menu class="menu" @menuItemClick="onClickMenuItem" @subMenuClick="onClickSubMenu">
        <Notes :items="menuStore.menu.items" />
        <Groups :menu="menuStore.menu.submenus" />
    </a-menu>
</template>

<script setup lang="ts">
import Notes from "./Notes.vue";
import Groups from "./Groups.vue";
import { useMenuStore } from "../../stores/menu";

const menuStore = useMenuStore();

const onClickMenuItem = (key: string) => {
    menuStore.getNoteContents(key);
    menuStore.setIsNoteEditing(true);
    menuStore.setEditingNoteKey(key);
};

const onClickSubMenu = (_key: string) => {
    menuStore.setIsNoteEditing(false);
    menuStore.setEditingNoteKey("");
};
</script>

<style scoped>
.menu {
    background: linear-gradient(-45deg, rgb(249, 249, 249), rgb(247, 240, 255));
}
</style>
