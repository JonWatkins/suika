// @ts-nocheck

import { useEffect, useState } from "suika";
import { getCurrentMode } from "./createRouter";

export const RouterProvider = ({ router }) => {
  const [currentPath, setCurrentPath] = useState(router.getFragment());
  const eventName = getCurrentMode() === "hash" ? "popstate" : "navigate";

  useEffect(() => {
    const onLocationChange = () => {
      setCurrentPath(router.getFragment());
    };

    window.addEventListener(eventName, onLocationChange);

    return () => {
      window.removeEventListener(eventName, onLocationChange);
    };
  }, []);

  const handler = router.getHandler(currentPath);
  return handler({ currentPath });
};
