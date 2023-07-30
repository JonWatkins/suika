// @ts-nocheck

import { createElement } from "suika";
import { Navbar } from "../components/Navbar";

export const NotFound = ({ currentPath }) => {
  return (
    <div id="home">
      <Navbar currentPath={currentPath} />
      <div className="container-xl mx-auto">
        <h1>404 Page not found</h1>
        <p>Unable to find path {currentPath}</p>
      </div>
    </div>
  );
};
