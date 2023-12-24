<template>
    <a-layout-content>
        <ActivityDescription v-if="isEditing" :editing="editing" />
        <ActivityList v-else :editing="editing" />
        {{ menuStore.root }}
        <!-- {{ isEditingDescription }} -->
    </a-layout-content>
</template>

<script setup lang="ts">
import { ref } from "vue";
import ActivityList from "./ActivityList.vue";
import ActivityDescription from "./ActivityDescription.vue";
import { useMenuStore } from "../../stores/menu";

const menuStore = useMenuStore();

const isEditing = ref(false);

const editing = (value: boolean) => {
    isEditing.value = value;
};
</script>
