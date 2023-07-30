// @ts-nocheck

import { createElement } from "suika";
import { Link } from "suika-router";
import { Counter } from "./Counter";

export const Hero = () => {
  return (
    <section className="hero">
      <div className="hero-container">
        <div className="hero-inner">
          <h1 className="hero-title">Lorem ipsum dolor</h1>
          <p className="hero-content">
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
            eiusmod tempor incididunt ut labore et dolore magna aliqua.
          </p>
          <div className="hero-links">
            <Link className="btn btn-md btn-primary mr-1.5" to="/about">
              Learn More
            </Link>
            <Counter key="counter" count={0} />
          </div>
        </div>
      </div>
    </section>
  );
};
