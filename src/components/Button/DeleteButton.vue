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
                <a-popconfirm
                    content="Are you sure you want to delete?"
                    position="bottom"
                    type="warning"
                    @before-ok="remove"
                >
                    <a-button type="outline" shape="round" status="danger">Delete</a-button>
                </a-popconfirm>
            </a-form-item>
        </a-form>
    </a-modal>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { reactive, ref, watch } from "vue";
import { Message } from "@arco-design/web-vue";
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
            menuStore.getGroupItems(newGroupKey);
        }
    }
);

const handleClick = async () => {
    visible.value = true;
    menuStore.getGroups();
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
    } else {
        await invoke<void>("remove_group", {
            parent_group_key: form.groupKey,
            key: item?.key,
        });
    }
    form.itemKey = "";
    menuStore.getMenu();
    menuStore.getGroups();
    Message.success({ content: `Delete ${item?.name} successfully!`, showIcon: true });
};
</script>
