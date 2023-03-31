import { Suika } from "../core/Suika";

export const mount = (Context, el: HTMLElement): Suika => {
  const instance = new Context();
  instance._mount(el);
  return instance;
};
