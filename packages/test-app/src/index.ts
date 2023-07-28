import "./styles.scss";

import { render } from "suika";
import { App } from "./App";

const root = document.getElementById("root");
render(App(), root as HTMLElement);
