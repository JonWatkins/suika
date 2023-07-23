import "./style.scss";

import { mount } from "suika";
import { App } from "./App";

const root = document.getElementById("app");
const instance = mount(App, root as HTMLElement);

export default instance;
