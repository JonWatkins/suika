import { Component, vNode, h } from "suika";

export class Home extends Component {
  render(): vNode {
    return (
      <div className="container-xl mx-auto">
        <section className="dark:bg-light-600">
          <div className="sm:container-xs md:container-md xl:container-lg mx-auto">
            <div className="py-4 px-2 lg:py-8 lg:px-4 mx-auto flex-col justify-center items-center">
              <h1 className="text-5xl">Lorem ipsum dolor</h1>
              <p className="text-xl mb-1.5 text-justify">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
                eiusmod tempor incididunt ut labore et dolore magna aliqua.
              </p>
              <div className="flex-row justify-center items-center">
                <a
                  className="bg-primary-500 py-0.5 px-1 rounded-md text-2xl mr-1.5 dark:text-light-500"
                  href="#"
                >
                  Learn More
                </a>
                <a
                  className="bg-secondary-500 py-0.5 px-1 rounded-md text-2xl mr-1.5 dark:text-light-500 py-0.5 px-1 rounded-md sm:none md:block"
                  href="#"
                >
                  Sign Up
                </a>
              </div>
            </div>
          </div>
        </section>
      </div>
    );
  }
}
