<template>
    <a-button @click="handleClick" status="danger">
        <span><icon-delete /> Delete </span>
    </a-button>
    <a-modal
        :visible="visible"
        title="Modal Form"
        @cancel="handleCancel"
        @before-ok="handleBeforeOk"
    >
        <a-form :model="form">
            <a-form-item field="path" label="Path">
                <a-select v-model="form.groupKey">
                    <a-option v-for="group in menuStore.groups" :value="group.key">
                        {{ group.path }}
                    </a-option>
                </a-select>
            </a-form-item>
            <a-form-item field="name" label="Name">
                <a-select v-model="form.itemKey">
                    <a-option v-for="item in menuStore.items" :value="item.key">
                        {{ item.name }}
                    </a-option>
                </a-select>
            </a-form-item>
            <a-form-item>
                <a-button type="primary" @click="remove">Delete</a-button>
            </a-form-item>
        </a-form>
    </a-modal>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { reactive, ref, watch } from "vue";
import { useMenuStore } from "../../stores/menu";
import { IconDelete } from "@arco-design/web-vue/es/icon";

const menuStore = useMenuStore();

const visible = ref(false);

const form = reactive({
    groupKey: "",
    itemKey: "",
});

watch(
    () => form.groupKey,
    async (newGroupKey) => {
        if (newGroupKey !== "") {
            menuStore.get_group_items(newGroupKey);
        }
    }
);

const handleClick = async () => {
    visible.value = true;
    menuStore.get_groups();
};

const handleBeforeOk = (done: Function) => {
    visible.value = false;
    form.groupKey = "";
    form.itemKey = "";
    done();
};

const handleCancel = () => {
    visible.value = false;
    form.groupKey = "";
    form.itemKey = "";
};

const remove = async () => {
    const item = Object.values(menuStore.items).find((item) => item.key === form.itemKey);
    if (item?.name.startsWith("Note")) {
        await invoke<void>("remove_note", {
            group_key: form.groupKey,
            key: item.key,
        });
    } else if (item?.name.startsWith("Group")) {
        await invoke<void>("remove_group", {
            parent_group_key: form.groupKey,
            key: item.key,
        });
    }
    form.itemKey = "";
    menuStore.get_menu();
    menuStore.get_groups();
};
</script>
