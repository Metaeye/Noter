<template>
    <a-layout style="height: 400px">
        <a-layout-header>Header</a-layout-header>
        <a-layout-content>
            <a-textarea
                default-value="This is the contents of the textarea. This is the contents of the textarea. This is the contents of the textarea."
                :auto-size="{ minRows: 5 }"
                style="margin-top: 20px"
            />
        </a-layout-content>
        <a-layout-footer>Footer</a-layout-footer>
    </a-layout>
</template>
