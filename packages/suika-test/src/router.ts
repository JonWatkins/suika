import { createRouter } from "suika-router";
import { Counter } from "./routes/Counter";

export const router = createRouter({
  mode: "hash",
  routes: [{ path: "/", component: Counter }],
});
