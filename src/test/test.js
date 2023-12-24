import * as fs from "fs";

let root = JSON.parse(fs.readFileSync("./root.json", "utf8"));

let title = root["title"];
let notes = root["notes"];
let groups = root["groups"];

function printNotes(notes) {
    Object.keys(notes).forEach((key) => {
        let todo_list = notes[key];
        console.log(key, ":", todo_list);
    });
}

function printGroup(group) {
    let title = group["title"];
    let notes = group["notes"];
    let groups = group["groups"];

    console.log("title:", title);
    console.log("notes:");
    printNotes(notes);
    console.log("groups:");
    Object.keys(groups).forEach((key) => {
        let group = groups[key];
        console.log(key, ":");
        printGroup(group);
    });
}

printGroup(root);
