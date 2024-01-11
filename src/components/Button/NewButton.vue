<template>
    <a-button @click="handleClick" type="outline">
        <span><icon-plus /> New </span>
    </a-button>
    <a-modal
        :visible="visible"
        title="Modal Form"
        @cancel="handleCancel"
        @before-ok="handleBeforeOk"
    >
        <a-form :model="form">
            <a-form-item field="name" label="Name">
                <input v-model="form.name" />
            </a-form-item>
            <a-form-item field="parentGroupKey" label="Path">
                <a-select class="custom-select" v-model="form.parentGroupKey">
                    <a-option v-for="group in menuStore.groups" :value="group.key">
                        {{ group.path }}
                    </a-option>
                </a-select>
            </a-form-item>
            <a-form-item field="type" label="Type">
                <a-radio-group v-model="form.type">
                    <a-radio value="note">Note</a-radio>
                    <a-radio value="group">Group</a-radio>
                </a-radio-group>
            </a-form-item>
            <a-form-item>
                <a-button type="outline" @click="insert" shape="round">
                    <span><icon-plus /> New </span>
                </a-button>
            </a-form-item>
        </a-form>
    </a-modal>
</template>

<script setup lang="ts">
import { reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { Message } from "@arco-design/web-vue";
import { useMenuStore } from "../../stores/menu";
import { IconPlus } from "@arco-design/web-vue/es/icon";

const menuStore = useMenuStore();

const visible = ref(false);

const form = reactive({
    name: "",
    type: "",
    parentGroupKey: "",
});

const handleClick = async () => {
    visible.value = true;
    menuStore.getGroups();
};

const handleBeforeOk = (done: Function) => {
    visible.value = false;
    form.name = "";
    form.type = "";
    form.parentGroupKey = "";
    done();
};

const handleCancel = () => {
    visible.value = false;
    form.name = "";
    form.type = "";
    form.parentGroupKey = "";
};

const insert = async () => {
    if (form.type === "note") {
        await invoke<void>("insert_note", {
            group_key: form.parentGroupKey,
            note_name: form.name,
        });
    } else {
        await invoke<void>("insert_group", {
            parent_group_key: form.parentGroupKey,
            group_name: form.name,
        });
    }
    menuStore.getMenu();
    menuStore.getGroups();
    Message.success({ content: `Create ${form.name} successfully!`, showIcon: true });
};
</script>
