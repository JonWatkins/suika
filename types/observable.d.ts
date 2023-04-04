export interface Observable {
  deleteProperty: Function;
  set: Function;
  get: Function;
}
export declare function observable(
  target: any,
  listener?: Function,
  tree?: Array<String>
): Observable | any;
