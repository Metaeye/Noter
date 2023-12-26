<template>
    <a-layout-content>
        <Description v-if="isEditing" :editing="editing" />
        <NoteContents v-else :editing="editing" />
    </a-layout-content>
</template>

<script setup lang="ts">
import { ref } from "vue";
import Description from "./Description.vue";
import NoteContents from "./NoteContents.vue";

const isEditing = ref(false);

const editing = (value: boolean) => {
    isEditing.value = value;
};
</script>
