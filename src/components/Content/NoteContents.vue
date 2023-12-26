<template>
    <a-list v-if="menuStore.isNoteEditing">
        <a-list-item v-for="(content, i) in menuStore.contents" :key="i">
            <a-list-item-meta
                class="left-align"
                :title="content[0]"
                :description="description(content[1])"
            />
            <template #actions>
                <icon-edit @click="beforeEdit(content, i)" />
                <icon-delete @click="beforeRemove(i)" />
            </template>
        </a-list-item>
        <a-list-item>
            <a-space size="medium">
                <input
                    :style="{ width: '320px' }"
                    placeholder="Please enter something"
                    v-model="newContentName"
                    @press-enter="beforeInsert"
                />
                <a-button type="primary" shape="round" @click="beforeInsert">
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

const newContentName = ref("");

const description = (origin: string): string => {
    if (origin.length > 50) {
        return origin.slice(0, 50) + "...";
    }
    return origin;
};

const beforeInsert = () => {
    menuStore.insertContent(menuStore.editingNoteKey, newContentName.value.trim());
    menuStore.getNoteContents(menuStore.editingNoteKey);
    newContentName.value = "";
};

const beforeRemove = (index: number) => {
    menuStore.removeContent(menuStore.editingNoteKey, index);
    menuStore.getNoteContents(menuStore.editingNoteKey);
};

const beforeEdit = (content: string[], index: number) => {
    props.editing(true);
    menuStore.setEditingContent(content);
    menuStore.setEditingContentIndex(index);
};
</script>

<style scoped>
.left-align {
    text-align: left;
}
</style>
