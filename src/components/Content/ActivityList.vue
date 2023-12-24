<template>
    <a-list v-if="menuStore.curNote">
        <a-list-item v-for="(value, i) in menuStore.curNote.contents" :key="i">
            <a-list-item-meta :title="value[0]" :description="value[1]" />
            <template #actions>
                <icon-edit @click="beforeEdit(value)" />
                <icon-delete @click="menuStore.removeContent(menuStore.curNote, i)" />
            </template>
        </a-list-item>
        <a-list-item>
            <a-space size="medium">
                <a-input
                    :style="{ width: '320px' }"
                    placeholder="Please enter something"
                    allow-clear
                    v-model="inputContent"
                    @press-enter="pushContent"
                />
                <a-button type="primary" shape="round" @click="pushContent">
                    <span><icon-plus /></span>
                </a-button>
            </a-space>
        </a-list-item>
    </a-list>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useMenuStore } from "../../stores/menu";
import { IconDelete, IconEdit, IconPlus } from "@arco-design/web-vue/es/icon";

const props = defineProps({
    editing: {
        type: Function,
        required: true,
    },
});

const menuStore = useMenuStore();

const inputContent = ref("");

const pushContent = () => {
    menuStore.pushContent(menuStore.curNote, [inputContent.value, "description"]);
    inputContent.value = "";
};

const beforeEdit = (content: string[]) => {
    menuStore.curContent = content;
    props.editing(true);
};
</script>
