<template>
    <a-list>
        <a-list-item v-for="(value, i) in contentStore.content" :key="i">
            <a-list-item-meta :title="value[0]" :description="value[1]" />
            <template #actions>
                <icon-edit @click="editingDescription" />
                <icon-delete @click="contentStore.removeActivity(i)" />
            </template>
        </a-list-item>
        <a-list-item v-if="contentStore.content">
            <a-space size="medium">
                <a-input
                    :style="{ width: '320px' }"
                    placeholder="Please enter something"
                    allow-clear
                    v-model="inputActivity"
                    @press-enter="pushActivity"
                />
                <a-button type="primary" shape="round" @click="pushActivity">
                    <span><icon-plus /></span>
                </a-button>
            </a-space>
        </a-list-item>
    </a-list>
</template>

<script setup lang="ts">
import { IconDelete, IconEdit, IconPlus } from "@arco-design/web-vue/es/icon";
import { useContentStore } from "../../stores/content";
import { ref } from "vue";

const props = defineProps({
    editingDescription: {
        type: Function,
        required: true,
    },
});

const contentStore = useContentStore();
const inputActivity = ref("");

const pushActivity = () => {
    contentStore.pushActivity(inputActivity.value);
    inputActivity.value = "";
};

const editingDescription = () => {
    props.editingDescription();
};
</script>
