// @ts-nocheck

import { createElement } from "suika";
import { Navbar } from "../components/Navbar";
import { Hero } from "../components/Hero";

export const Home = ({ currentPath }) => {
  return (
    <div id="home">
      <Navbar currentPath={currentPath} />
      <div className="container-xl mx-auto">
        <Hero />
      </div>
    </div>
  );
};
