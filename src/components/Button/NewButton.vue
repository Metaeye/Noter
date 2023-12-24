<template>
    <a-button @click="handleClick" status="success">
        <span><icon-plus /> New </span>
    </a-button>
    <a-modal
        :visible="visible"
        title="Modal Form"
        @cancel="handleCancel"
        @before-ok="handleBeforeOk"
    >
        <a-form :model="form">
            <a-form-item field="title" label="Title">
                <a-input v-model="form.title" />
            </a-form-item>
            <a-form-item field="type" label="Type">
                <a-select v-model="form.type">
                    <a-option value="note">Note</a-option>
                    <a-option value="group">Group</a-option>
                </a-select>
            </a-form-item>
            <a-form-item field="level" label="Level">
                <a-select v-model="form.level">
                    <a-option v-for="group in menuStore.groupList" :value="group.key">
                        {{ group.title }}
                    </a-option>
                </a-select>
            </a-form-item>
        </a-form>
    </a-modal>
</template>

<script setup lang="ts">
import { reactive, ref } from "vue";
import { IconPlus } from "@arco-design/web-vue/es/icon";
import { useMenuStore } from "../../stores/menu";

const menuStore = useMenuStore();

const visible = ref(false);

const form = reactive({
    title: "",
    type: "",
    level: "",
});

const handleClick = () => {
    visible.value = true;
};

const handleBeforeOk = (done: Function) => {
    switch (form.type) {
        case "note": {
            menuStore.pushNote(form.level, form.title);
            break;
        }
        case "group": {
            menuStore.pushGroup(form.level, form.title);
            break;
        }
    }
    visible.value = false;
    done();
};

const handleCancel = () => {
    visible.value = false;
};
</script>
