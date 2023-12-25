import { ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";

const time_stamp = () => {
    return new Date().getTime().toString();
};

const find =
    (what: string) =>
    (where: any): any => {
        return where.key === what
            ? where
            : where.items.find((item: any) => item.key === what) ||
                  where.submenus
                      .map((submenu: any) => find(what)(submenu))
                      .find((value: any) => value !== undefined);
    };

const toTree = (root: any) => {
    const convertToTree = (node: any) => {
        const tree: any = {
            key: node.key,
            title: node.title,
            children: [],
        };
        node.submenus.forEach((submenu: any) => {
            tree.children.push(convertToTree(submenu));
        });
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
    tree.children.forEach((child: any) => {
        toList(child, newPath, result);
    });
    // if (tree.children) {
    //     tree.children.forEach((child: any) => {
    //         toList(child, newPath, result);
    //     });
    // }
    return result;
};

export const useMenuStore = defineStore("menu", () => {
    const initialize = () => {
        return JSON.parse(`{
            "key": "root",
            "title": "root",
            "items": [
                {
                    "key": "1703339815803139800",
                    "title": "gift"
                }
            ],
            "submenus": [
                {
                    "key": "1703339815803137900",
                    "title": "math",
                    "items": [
                        {
                            "key": "1703339815803137800",
                            "title": "math"
                        }
                    ],
                    "submenus": [
                        {
                            "key": "1703339815803135200",
                            "title": "algebra",
                            "items": [
                                {
                                    "key": "1703339815803135000",
                                    "title": "algebra"
                                }
                            ],
                            "submenus": [
                                {
                                    "key": "1703339815803122800",
                                    "title": "add",
                                    "items": [
                                        {
                                            "key": "1703339815803119900",
                                            "title": "add"
                                        }
                                    ],
                                    "submenus": []
                                }
                            ]
                        }
                    ]
                }
            ]
        }`);
    };

    const root: any = ref(initialize());

    const get_menu = async () => {
        root.value = JSON.parse(await invoke<string>("get_menu"));
    };

    const curNote = ref();
    const curContent = ref();

    const groupList = ref(toList(toTree(root.value)));

    const find_in_root = (key: string) => {
        return find(key)(root.value);
    };

    const children = (parent: any) => {
        const items = parent.items.map((item: any) => {
            return {
                key: item.key,
                title: ["notes", item.title].join("/"),
            };
        });
        const submenus = parent.submenus.map((submenu: any) => {
            return {
                key: submenu.key,
                title: ["groups", submenu.title].join("/"),
            };
        });

        return [...items, ...submenus];
    };

    const pushNote = (parent: string, title: string) => {
        const key = time_stamp();
        const value = {
            key,
            parent,
            title,
            contents: [],
        };
        find_in_root(parent).notes[key] = value;
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
        initialize,
        get_menu,
    };
});
