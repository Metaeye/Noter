import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import WelcomeView from "../views/WelcomeView.vue";

export const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: "/", component: WelcomeView },
        { path: "/home", component: HomeView },
    ],
});
