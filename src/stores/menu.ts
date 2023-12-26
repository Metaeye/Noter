import { ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";

export const useMenuStore = defineStore("menu", () => {
    const menu = ref({
        key: "",
        title: "",
        items: [],
        submenus: [],
    });

    const groups = ref([
        {
            key: "",
            path: "",
        },
    ]);

    const items = ref([
        {
            key: "",
            name: "",
        },
    ]);

    const get_menu = async () => {
        await invoke<string>("get_menu").then((res) => {
            menu.value = JSON.parse(res);
        });
    };

    const get_groups = async () => {
        await invoke<string>("get_groups").then((res) => {
            groups.value = JSON.parse(res);
        });
    };

    const get_group_items = async (groupKey: string) => {
        await invoke<string>("get_group_items", { group_key: groupKey }).then((res) => {
            items.value = JSON.parse(res);
        });
    };

    return {
        menu,
        groups,
        items,
        get_menu,
        get_groups,
        get_group_items,
    };
});
