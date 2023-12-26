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

    const contents = ref([]);

    const editingContent = ref(["", ""]);

    const editingContentIndex = ref(0);

    const isNoteEditing = ref(false);

    const editingNoteKey = ref("");

    const setIsNoteEditing = (isEditing: boolean) => {
        isNoteEditing.value = isEditing;
    };

    const setEditingNoteKey = (noteKey: string) => {
        editingNoteKey.value = noteKey;
    };

    const setEditingContent = (content: string[]) => {
        editingContent.value = content;
    };

    const setEditingContentIndex = (index: number) => {
        editingContentIndex.value = index;
    };

    const getMenu = async () => {
        await invoke<string>("get_menu").then((res) => {
            menu.value = JSON.parse(res);
        });
    };

    const getGroups = async () => {
        await invoke<string>("get_groups").then((res) => {
            groups.value = JSON.parse(res);
        });
    };

    const getGroupItems = async (groupKey: string) => {
        await invoke<string>("get_group_items", { group_key: groupKey }).then((res) => {
            items.value = JSON.parse(res);
        });
    };

    const getNoteContents = async (noteKey: string) => {
        await invoke<string>("get_note_contents", { note_key: noteKey }).then((res) => {
            contents.value = JSON.parse(res);
        });
    };

    const insertContent = async (noteKey:string, contentName: string) => {
        await invoke("insert_content", {
            note_key: noteKey,
            json_content: JSON.stringify([contentName, "Description"]),
        });
    };

    const removeContent = async (noteKey: string, index: number) => {
        await invoke<string>("remove_content", { note_key: noteKey, index: index });
    };

    const updateContent = async (noteKey: string, index: number, jsonContent: string[]) => {
        await invoke<string>("update_content", {
            note_key: noteKey,
            index: index,
            json_content: JSON.stringify(jsonContent),
        });
    };

    return {
        menu,
        groups,
        items,
        contents,
        isNoteEditing,
        editingNoteKey,
        editingContent,
        editingContentIndex,
        getMenu,
        getGroups,
        getGroupItems,
        getNoteContents,
        insertContent,
        removeContent,
        updateContent,
        setIsNoteEditing,
        setEditingNoteKey,
        setEditingContent,
        setEditingContentIndex,
    };
});
