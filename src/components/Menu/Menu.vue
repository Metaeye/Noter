<template>
    <a-menu @menuItemClick="onClickMenuItem" @subMenuClick="onClickMenuItem">
        <Notes :notes="menuStore.root.notes" />
        <Groups :groups="menuStore.root.groups" />
    </a-menu>
    <a-space v-if="props.collapsed" size="medium" class="button-container">
        <NewButton />
        <DelButton />
    </a-space>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";
import { useMenuStore } from "../../stores/menu";
import { useContentStore } from "../../stores/content";
import NewButton from "../Button/NewButton.vue";
import DelButton from "../Button/DelButton.vue";
import Groups from "./Groups.vue";
import Notes from "./Notes.vue";

const props = defineProps({
    collapsed: {
        type: Boolean,
        required: true,
    },
});

const menuStore = useMenuStore();

const contentStore = useContentStore();

const onClickMenuItem = (key: string) => {
    const result: any = menuStore.find_in_root(key);
    contentStore.setContent(
        Object.keys(result).find((k) => k === "content") === undefined
            ? undefined
            : result["content"]
    );
    Message.info({ content: `You select ${key}`, showIcon: true });
};
</script>

<style scoped>
.button-container {
    display: flex;
    justify-content: center;
}
</style>
