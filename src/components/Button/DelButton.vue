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
            <a-form-item field="level" label="Level">
                <a-select v-model="form.level">
                    <a-option v-for="group in menuStore.groupList" :value="group.key">
                        {{ group.title }}
                    </a-option>
                </a-select>
            </a-form-item>
            <a-form-item field="title" label="Title">
                <a-select v-model="form.title">
                    <a-option
                        v-for="child in menuStore.children(menuStore.find_in_root(form.level))"
                        :value="child.key"
                    >
                        {{ child.title }}
                    </a-option>
                </a-select>
            </a-form-item>
        </a-form>
    </a-modal>
</template>

<script setup lang="ts">
import { reactive, ref } from "vue";
import { IconDelete } from "@arco-design/web-vue/es/icon";
import { useMenuStore } from "../../stores/menu";

const menuStore = useMenuStore();

const visible = ref(false);

const form = reactive({
    level: "root",
    title: "root",
});

const handleClick = () => {
    visible.value = true;
};

const handleBeforeOk = (done: Function) => {
    menuStore.remove(form.level, form.title);
    visible.value = false;
    done();
};

const handleCancel = () => {
    visible.value = false;
};
</script>
