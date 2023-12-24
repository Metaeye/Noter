"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var fs = require("fs");
var find = function (what) {
    return function (where) {
        if (what === "root") {
            return where;
        }
        var note_key = Object.keys(where.notes).find(function (k) { return k === what; });
        if (note_key !== undefined) {
            return where.notes[note_key];
        }
        var group_key = Object.keys(where.groups).find(function (k) { return k === what; });
        if (group_key !== undefined) {
            return where.groups[group_key];
        }
        return Object.values(where.groups)
            .map(function (group) { return find(what)(group); })
            .find(function (value) { return value !== undefined; });
    };
};
var toTree = function (root) {
    var convertToTree = function (node) {
        var tree = {
            key: node.key,
            title: node.title,
            children: [],
        };
        if (node.groups) {
            Object.values(node.groups).forEach(function (group) {
                tree.children.push(convertToTree(group));
            });
        }
        return tree;
    };
    return convertToTree(root);
};

const printTree = (tree, level = 0) => {
    console.log(`${' '.repeat(level)}${tree.title}`);
    if (tree.children) {
        tree.children.forEach(child => printTree(child, level + 1));
    }
}

const toList = (tree) => {
    const list = [];
    const convertToList = (node) => {
        list.push({
            key: node.key,
            title: node.title,
        });
        if (node.children) {
            node.children.forEach(child => convertToList(child));
        }
    };
    convertToList(tree);
    return list;
}

var where = JSON.parse(fs.readFileSync("./root.json", "utf8"));
var tree = toTree(where);
var list = toList(tree);
console.log(list);
