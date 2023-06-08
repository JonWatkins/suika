import { createRouter } from "suika-router";
import { Home } from "./routes/Home";
import { Counter } from "./routes/Counter";

export const router = createRouter({
  mode: "hash",
  routes: [
    { path: "/", component: Home },
    { path: "/counter", component: Counter }
  ],
});
