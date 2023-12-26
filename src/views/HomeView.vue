<template>
    <a-layout class="layout-demo">
        <a-layout-sider theme="light" :resize-directions="['right']" :width="220">
            <div class="logo" />
            <Menu />
            <br />
            <a-space size="medium" class="button-container">
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
import Menu from "../components/Menu/Menu.vue";
import Content from "../components/Content/Content.vue";
import NewButton from "../components/Button/NewButton.vue";
import DeleteButton from "../components/Button/DeleteButton.vue";
</script>

<style scoped>
.layout-demo {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: linear-gradient(-45deg, rgb(249, 249, 249), rgb(247, 240, 255));
    border: 1px solid var(--color-border);
}
.layout-demo :deep(.arco-layout-sider) {
    width: 200px;
    min-width: 200px;
    max-width: 750px;
    background: linear-gradient(-45deg, rgb(249, 249, 249), rgb(247, 240, 255));
}
.layout-demo :deep(.arco-layout-sider-light) .logo {
    height: 32px;
    margin: 12px 8px;
    background: linear-gradient(-45deg, rgb(249, 249, 249), rgb(247, 240, 255));
}
.layout-demo :deep(.arco-layout-header) {
    height: 32px;
    line-height: 64px;
    background: linear-gradient(-45deg, rgb(247, 240, 255), rgb(249, 249, 249));
}
.layout-demo :deep(.arco-layout-content) {
    font-weight: 400;
    font-size: 14px;
    background: linear-gradient(-45deg, rgb(247, 240, 255), rgb(249, 249, 249));
}
.layout-demo :deep(.arco-layout-content) {
    display: flex;
    flex-direction: column;
    justify-content: center;
    font-size: 16px;
    font-stretch: condensed;
    text-align: center;
}
.button-container {
    display: flex;
    justify-content: center;
}
</style>
