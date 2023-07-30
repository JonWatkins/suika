// @ts-nocheck

import { createElement } from "suika";
import { createRouter, RouterProvider } from "suika-router";
import { Home } from "./routes/Home";
import { About } from "./routes/About";
import { NotFound } from "./routes/NotFound";

const router = createRouter({
  mode: "hash",
  NotFound,
  routes: [
    { path: "/", component: Home },
    { path: "/about", component: About },
  ],
});

export const App = () => {
  return (
    <div id="container">
      <RouterProvider router={router} />
    </div>
  );
};
