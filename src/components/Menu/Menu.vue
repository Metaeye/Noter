<template>
    <a-menu @menuItemClick="onClickMenuItem" @subMenuClick="onClickMenuItem">
        <Notes :notes="menuStore.root.notes" />
        <Groups :groups="menuStore.root.groups" />
    </a-menu>
    <a-space v-if="props.collapsed" size="medium" class="button-container">
        <NewButton />
        <DeleteButton />
    </a-space>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";
import { useMenuStore } from "../../stores/menu";
import NewButton from "../Button/NewButton.vue";
import DeleteButton from "../Button/DeleteButton.vue";
import Groups from "./Groups.vue";
import Notes from "./Notes.vue";

const props = defineProps({
    collapsed: {
        type: Boolean,
        required: true,
    },
});

const menuStore = useMenuStore();

const onClickMenuItem = (key: string) => {
    const value = menuStore.find_in_root(key);
    menuStore.curNote = Object.keys(value).includes("contents") ? value : undefined;
    Message.info({ content: `You select ${key}`, showIcon: true });
};
</script>

<style scoped>
.button-container {
    display: flex;
    justify-content: center;
}
</style>
