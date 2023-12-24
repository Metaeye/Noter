import { ref } from "vue";
import { defineStore } from "pinia";

export const useContentStore = defineStore("content", () => {
    const content = ref();

    const setContent = (newContent: Array<Array<string>>) => {
        content.value = newContent;
    };

    const pushActivity = (activity: string) => {
        content.value.push([activity, ""]);
    };

    const removeActivity = (index: number) => {
        content.value.splice(index, 1);
    };

    const editDescription = (index: number, description: string) => {
        content.value[index][1] = description;
    };

    return { content, setContent, pushActivity, removeActivity, editDescription };
});
