<template>
    <a-layout-content>
        <ActivityDescription v-if="isEditingDescription" />
        <ActivityList v-else :editingDescription="editingDescription" />
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

const isEditingDescription = ref(false);

const editingDescription = () => {
    isEditingDescription.value = true;
};
</script>
