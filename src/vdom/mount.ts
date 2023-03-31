import { NekoJS } from "../core/NekoJS";

export const mount = (Context, el: HTMLElement): NekoJS => {
  const instance = new Context();
  instance._mount(el);
  return instance;
};
