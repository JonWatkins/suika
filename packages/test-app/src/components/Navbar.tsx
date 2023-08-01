// @ts-nocheck

import { createElement } from "suika";
import { Link } from "suika-router";

export const Navbar = ({ currentPath }) => {
  return (
    <nav className="navbar">
      <div className="navbar-inner">
        <Link to="/" className="navbar-brand">
          Suika
        </Link>

        <ul className="navbar-nav">
          <li>
            <Link
              to="/"
              className={`navbar-link ${
                currentPath === "/" ? "is-active" : ""
              }`}
            >
              Home
            </Link>
          </li>
          <li>
            <Link
              to="/about"
              className={`navbar-link ${
                currentPath === "/about" ? "is-active" : ""
              }`}
            >
              About
            </Link>
          </li>
          <li>
            <Link to="/broken-link" className="navbar-link">
              404 Error Page
            </Link>
          </li>
        </ul>
      </div>
    </nav>
  );
};
