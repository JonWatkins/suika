// @ts-nocheck

import { createElement, useState } from "suika";
import { Button } from "suika-ui";

export const Counter = ({ count }) => {
  const [currentCount, setCount] = useState(count);

  return (
    <Button onClick={() => setCount(currentCount + 1)}>
      Clicked {currentCount} times!
    </Button>
  );
};
