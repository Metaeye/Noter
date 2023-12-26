<template>
    <a-layout style="height: 400px">
        <a-layout-content>
            <a-space class="back-button">
                <a-button type="outline" shape="round" @click="beforeBack">
                    <template #icon>
                        <icon-arrow-left />
                    </template>
                </a-button>
            </a-space>
            <a-typography :style="{ marginTop: '-40px' }">
                <a-typography-title v-if="!isActivityEditing" @click="isActivityEditing = true">
                    {{ menuStore.editingContent[0] }}
                </a-typography-title>
                <a-textarea
                    v-else
                    v-model="menuStore.editingContent[0]"
                    @blur="isActivityEditing = false"
                    :auto-size="{ minRows: 1, maxRows: 1 }"
                    style="margin-top: 20px"
                />
                <a-typography-paragraph
                    v-if="!isDescriptionEditing"
                    @click="isDescriptionEditing = true"
                >
                    {{ menuStore.editingContent[1] }}
                </a-typography-paragraph>
                <a-textarea
                    v-else
                    v-model="menuStore.editingContent[1]"
                    @blur="isDescriptionEditing = false"
                    :auto-size="{ minRows: 5 }"
                    style="margin-top: 20px"
                />
            </a-typography>
        </a-layout-content>
        <a-layout-footer>Footer</a-layout-footer>
    </a-layout>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { IconArrowLeft } from "@arco-design/web-vue/es/icon";
import { useMenuStore } from "../../stores/menu";

const props = defineProps({
    editing: {
        type: Function,
        required: true,
    },
});

const menuStore = useMenuStore();

const isActivityEditing = ref(false);

const isDescriptionEditing = ref(false);

const beforeBack = () => {
    props.editing(false);
    menuStore.updateContent(
        menuStore.editingNoteKey,
        menuStore.editingContentIndex,
        menuStore.editingContent
    );
    menuStore.setEditingContentIndex(-1);
    menuStore.setEditingContent(["", ""]);
};
</script>

<style scoped>
.back-button {
    position: relative;
    top: 10px;
    left: 20px;
}
</style>
