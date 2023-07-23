import { Component, vNode, h, reactive } from "suika";
import { Button } from "suika-ui";

export class Counter extends Component {
  state = reactive({
    count: 0,
  });
  render(): vNode {
    return (
      <div className="container-xl mx-auto">
        <h1
          dangerouslySetHtml={{
            __html: `Count: <em>${this.state.value.count}</em>`,
          }}
        />
        <Button onclick={() => this.state.value.count++}>Inc</Button>
      </div>
    );
  }
}
