// @ts-nocheck

import { createElement, useState, useEffect } from "suika";
import { Button } from "suika-ui";

export const Counter = ({ count }) => {
  const [currentCount, setCount] = useState(count);

  useEffect(() => {
    console.log(currentCount);
  }, [currentCount]);

  return (
    <Button
      className="btn btn-md btn-primary"
      onClick={() => setCount(currentCount + 1)}
    >
      Clicked {currentCount} times!
    </Button>
  );
};
