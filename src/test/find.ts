import * as fs from "fs";

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

const where = JSON.parse(fs.readFileSync("./root.json", "utf8"));
const tree = toTree(where);
console.log(tree);
