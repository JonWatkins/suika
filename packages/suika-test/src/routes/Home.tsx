import { Component, vNode, h } from "suika";

export class Home extends Component {
  render(): vNode {
    return (
      <div className="container-xl mx-auto">
        <section className="dark:bg-light-600 mb-1.5">
          <div className="sm:container-xs md:container-md xl:container-lg mx-auto">
            <div className="py-4 px-2 lg:py-8 lg:px-4 mx-auto flex-col text-center">
              <h1 className="text-5xl">Lorem ipsum dolor</h1>
              <p className="text-xl mb-1.5">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
                eiusmod tempor incididunt ut labore et dolore magna aliqua.
              </p>
              <div className="flex-row justify-center items-center">
                <a className="btn btn-primary mr-1.5" href="#">
                  Learn More
                </a>
                <a className="btn btn-secondary" href="#">
                  Sign Up
                </a>
              </div>
            </div>
          </div>
        </section>

        <section className="flex-col md:flex-row m-1.5 xl:m-0">
          <div className="card mb-1.5 md:mb-0 md:mr-1.5">
            <div className="card-header">Lorem ipsum</div>
            <div className="card-body">
              <h2 className="card-title">Lorem ipsum</h2>
              <p className="card-text">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
                eiusmod tempor incididunt ut labore et dolore magna aliqua.
              </p>
            </div>
          </div>
          <div className="card">
            <div className="card-header">Lorem ipsum</div>
            <div className="card-body">
              <h2 className="card-title">Lorem ipsum</h2>
              <p className="card-text">
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
                eiusmod tempor incididunt ut labore et dolore magna aliqua.
              </p>
            </div>
          </div>
        </section>
      </div>
    );
  }
}
