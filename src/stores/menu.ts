import { ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/tauri";

const initialize = async (root: any) => {
    // root.value = JSON.parse(await invoke("initialize_menu", {}));
    root.value = JSON.parse(`{
        "key": "root",
        "title": "root",
        "notes": {
            "1703339815803139800": {
                "key": "1703339815803139800",
                "parent": "root",
                "title": "gift",
                "contents": [
                    ["activity1", "description1"],
                    ["activity2", "description2"]
                ]
            }
        },
        "groups": {
            "1703339815803137900": {
                "key": "1703339815803137900",
                "title": "math",
                "notes": {
                    "1703339815803137800": {
                        "key": "1703339815803137800",
                        "parent": "1703339815803137900",
                        "title": "math",
                        "contents": [
                            ["activity1", "description1"],
                            ["activity2", "description2"]
                        ]
                    }
                },
                "groups": {
                    "1703339815803135200": {
                        "key": "1703339815803135200",
                        "title": "algebra",
                        "notes": {
                            "1703339815803135000": {
                                "key": "1703339815803135000",
                                "parent": "1703339815803135200",
                                "title": "algebra",
                                "contents": [
                                    ["activity1", "description1"],
                                    ["activity2", "description2"]
                                ]
                            }
                        },
                        "groups": {
                            "1703339815803122800": {
                                "key": "1703339815803122800",
                                "title": "add",
                                "notes": {
                                    "1703339815803119900": {
                                        "key": "1703339815803119900",
                                        "parent": "1703339815803122800",
                                        "title": "add",
                                        "contents": [
                                            ["activity1", "description1"],
                                            ["activity2", "description2"]
                                        ]
                                    }
                                },
                                "groups": {}
                            }
                        }
                    }
                }
            }
        }
    }`);
};

const time_stamp = () => {
    return new Date().getTime().toString();
};

const find =
    (what: string) =>
    (where: any): any => {
        if (what === "root") {
            return where;
        }
        const note_key = Object.keys(where.notes).find((k) => k === what);
        if (note_key !== undefined) {
            return where.notes[note_key];
        }
        const group_key = Object.keys(where.groups).find((k) => k === what);
        if (group_key !== undefined) {
            return where.groups[group_key];
        }
        return Object.values(where.groups)
            .map((group) => find(what)(group))
            .find((value) => value !== undefined);
    };

const toTree = (root: any) => {
    const convertToTree = (node: any) => {
        const tree: any = {
            key: node.key,
            title: node.title,
            children: [],
        };
        if (node.groups) {
            Object.values(node.groups).forEach((group: any) => {
                tree.children.push(convertToTree(group));
            });
        }
        return tree;
    };
    return convertToTree(root);
};

const toList = (tree: any, path: string[] = [], result: any[] = []) => {
    const newPath = [...path, tree.title];
    result.push({
        ...tree,
        title: newPath.join("/"),
    });
    if (tree.children) {
        tree.children.forEach((child: any) => {
            toList(child, newPath, result);
        });
    }
    return result;
};

export const useMenuStore = defineStore("menu", () => {
    const root: any = ref();
    initialize(root);

    const curNote = ref();
    const curContent = ref();

    const groupList = ref(toList(toTree(root.value)));

    const find_in_root = (key: string) => {
        return find(key)(root.value);
    };

    const children = (parent: any) => {
        const children: any[] = [];
        Object.values(parent.notes).forEach((note: any) => {
            children.push({
                key: note.key,
                title: ["notes", note.title].join("/"),
            });
        });
        Object.values(parent.groups).forEach((group: any) => {
            children.push({
                key: group.key,
                title: ["groups", group.title].join("/"),
            });
        });
        return children;
    };

    const pushNote = (parent: string, title: string) => {
        const key = time_stamp();
        find_in_root(parent).notes[key] = {
            key,
            parent,
            title,
            contents: [],
        };
        // todo: send notes to backend to save
    };

    const pushGroup = (parent: string, title: string) => {
        const key = time_stamp();
        find_in_root(parent).groups[key] = {
            key,
            title,
            notes: {},
            groups: {},
        };
        groupList.value = toList(toTree(root.value));
        // todo: send groups to backend to save
    };

    const remove = (parent: string, key: string) => {
        const node = find_in_root(parent);
        if (node.notes[key]) {
            delete node.notes[key];
        } else if (node.groups[key]) {
            delete node.groups[key];
        }
        // todo: send notes or groups to backend to save
    };

    const pushContent = (note: any, content: string[]) => {
        note.contents.push(content);
        // todo: send contents to backend to save
    };

    const removeContent = (note: any, index: number) => {
        note.contents.splice(index, 1);
        // todo: send contents to backend to save
    };

    return {
        root,
        groupList,
        curNote,
        curContent,
        find_in_root,
        pushNote,
        pushGroup,
        remove,
        children,
        pushContent,
        removeContent,
    };
});
