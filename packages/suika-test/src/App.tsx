import { Component, vNode, h } from "suika";
import { RouterView, RouterLink } from "suika-router";
import { router } from "./router";

export class App extends Component {
  render(): vNode {
    return (
      <div id="container">
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/counter">Counter</RouterLink>
        <RouterLink to="/404">404 Error</RouterLink>
        <RouterView router={router} />
      </div>
    );
  }
}
