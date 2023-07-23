import { Component, vNode, h } from "suika";
import { RouterView, RouterLink } from "suika-router";
import { router } from "./router";

export class App extends Component {
  render(): vNode {
    return (
      <div id="container">
        <nav className="dark:bg-dark-500">
          <div className="container-xl mx-auto flex">
            <RouterLink
              to="/"
              className="text-2xl py-0.5 flex-grow text-primary-500 dark:text-light-500"
            >
              Suika
            </RouterLink>
            <ul className="flex">
              <li className="ml-1.5">
                <RouterLink
                  to="/"
                  className="inline-block text-primary-500 dark:text-light-500 dark:hover:text-light-700 hover:text-primary-800 py-1 pr-0.5"
                >
                  Home
                </RouterLink>
              </li>
              <li className="ml-1.5">
                <RouterLink
                  to="/counter"
                  className="inline-block text-primary-500 dark:text-light-500 dark:hover:text-light-700 hover:text-primary-800 py-1 pr-0.5"
                >
                  Counter
                </RouterLink>
              </li>
              <li className="ml-1.5">
                <RouterLink
                  to="/404"
                  className="inline-block text-primary-500 dark:text-light-500 dark:hover:text-light-700 hover:text-primary-800 py-1 pr-0.5"
                >
                  404 Error
                </RouterLink>
              </li>
            </ul>
          </div>
        </nav>
        <RouterView router={router} />
      </div>
    );
  }
}
