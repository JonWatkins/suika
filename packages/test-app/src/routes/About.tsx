// @ts-nocheck

import { createElement } from "suika";
import { Navbar } from "../components/Navbar";

export const About = ({ currentPath }) => {
  return (
    <div id="about">
      <Navbar currentPath={currentPath} />
    </div>
  );
};
