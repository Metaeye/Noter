<template>
    <a-layout class="layout-demo">
        <a-layout-sider
            theme="light"
            :width="220"
            :collapsible="true"
            :collapsed="collapsed"
            @collapse="onCollapse"
        >
            <div class="logo" />
            <Menu />
            <a-space v-if="!collapsed" size="medium" class="button-container">
                <NewButton />
                <DeleteButton />
            </a-space>
        </a-layout-sider>
        <a-layout :has-sider="false">
            <Content />
        </a-layout>
    </a-layout>
</template>

<script setup lang="ts">
import { ref } from "vue";
// import { useMenuStore } from "../stores/menu";
import Menu from "../components/Menu/Menu.vue";
import Content from "../components/Content/Content.vue";
import NewButton from "../components/Button/NewButton.vue";
import DeleteButton from "../components/Button/DeleteButton.vue";

const collapsed = ref(true);

// const menuStore = useMenuStore();

const onCollapse = (val: boolean) => {
    collapsed.value = val;
};
</script>

<style scoped>
.layout-demo {
    height: 500px;
    background: var(--color-fill-2);
    border: 1px solid var(--color-border);
}
.layout-demo :deep(.arco-layout-sider) .logo {
    height: 32px;
    margin: 12px 8px;
    background: rgba(255, 255, 255, 0.2);
}
.layout-demo :deep(.arco-layout-sider-light) .logo {
    background: var(--color-fill-2);
}
.layout-demo :deep(.arco-layout-header) {
    height: 32px;
    line-height: 64px;
    background: var(--color-bg-3);
}
.layout-demo :deep(.arco-layout-footer) {
    height: 32px;
    color: var(--color-text-2);
    font-weight: 400;
    font-size: 14px;
    line-height: 48px;
}
.layout-demo :deep(.arco-layout-content) {
    color: var(--color-text-2);
    font-weight: 400;
    font-size: 14px;
    background: var(--color-bg-3);
}
.layout-demo :deep(.arco-layout-footer),
.layout-demo :deep(.arco-layout-content) {
    display: flex;
    flex-direction: column;
    justify-content: center;
    color: var(--color-white);
    font-size: 16px;
    font-stretch: condensed;
    text-align: center;
}
.button-container {
    display: flex;
    justify-content: center;
}
</style>
