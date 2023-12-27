<template>
    <div class="login10">
        <div class="login10-bg"></div>
        <div class="login10-container">
            <div class="login10-container-head">Welcome to Noter!</div>
            <router-link to="/home" @click="menuStore.getMenu">
                <span> Start <icon-pen-fill /></span>
            </router-link>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useMenuStore } from "../stores/menu";
import { IconPenFill } from "@arco-design/web-vue/es/icon";

const menuStore = useMenuStore();
</script>

<style lang="less" scoped>
span {
    font-size: 20px; /* 调整为你想要的字号大小 */
}
.login10 {
    overflow: hidden;
    height: 100vh;
    position: relative;
    font-family: sans-serif;

    &-bg {
        position: absolute;
        width: 100%;
        height: 100%;
        background: linear-gradient(-45deg, rgb(249, 249, 249), rgb(234, 226, 243));
        background-size: cover;
        z-index: -100;
    }

    &-container {
        background: rgba(255, 255, 255, 0.6);
        margin: 60px auto 0;
        width: 400px;
        border-radius: 16px;
        padding: 40px;
        display: flex;
        align-items: center;
        flex-direction: column;

        border-top: 2px solid rgba(255, 255, 255, 0.3);
        border-left: 2px solid rgba(255, 255, 255, 0.3);
        box-shadow: 2px 2px 10px rgba(0, 0, 0, 0.2);
        &-head {
            font-size: 30px;
            margin: 40px 0;
        }
        &-wrap {
            width: 100%;
            height: 40px;
            background: rgba(245, 246, 250, 1);
            border-radius: 40px;
            margin-bottom: 20px;
            display: grid;
            grid-template-columns: 15% 86%;
            input {
                outline: none;
                border: none;
                background: none;
                font-size: 16px;
                &::placeholder {
                    font-size: 16px;
                }
            }

            i {
                line-height: 40px;
                text-align: center;
            }
        }

        &-access {
            margin-bottom: 20px;
            width: 100%;
            padding: 0 0.4rem;
            display: flex;
            justify-content: flex-end;
            span {
                margin-left: 8px;
            }
        }

        &-action {
            margin-bottom: 20px;
        }

        &-or {
            margin-bottom: 20px;
            display: flex;
            width: 100%;
            &:before,
            &:after {
                content: "";
                border-bottom: 1px solid black;
                flex: 1 1;
                margin: auto;
            }
        }

        &-signup {
            margin: 20px 0;
        }
    }
}
</style>
