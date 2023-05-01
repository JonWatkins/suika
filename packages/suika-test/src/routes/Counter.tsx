import { Component, vNode, h, Fragment, reactive } from "suika";
import { Button } from "suika-ui";

export class Counter extends Component {
  state = reactive({
    count: 0,
  });
  render(): vNode {
    return (
      <>
        <h1
          dangerouslySetHtml={{
            __html: `Count: <em>${this.state.value.count}</em>`,
          }}
        />
        <Button onclick={() => this.state.value.count++}>Inc</Button>
      </>
    );
  }
}
