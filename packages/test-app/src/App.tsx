// @ts-nocheck

import { createElement, useEffect, useState } from "suika";
import { Button } from "../../suika-ui/dist";

const Counter = ({ count }) => {
  const [currentCount, setCount] = useState(count);

  useEffect(() => {
    console.log(currentCount);
  }, [count]);

  return (
    <Button
      className="btn btn-md btn-primary"
      onClick={() => setCount(currentCount + 1)}
    >
      Clicked {currentCount} times!
    </Button>
  );
};

export const App = () => {
  return (
    <div className="xl:container-lg mx-auto">
      <h1 style={{ color: "blue", fontWeight: "bold" }}>Hello World</h1>
      <Counter count={0} />
    </div>
  );
};
