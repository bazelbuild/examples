// This file is copied from https://github.com/remix-run/react-router/tree/main/packages/react-router-dev/config/defaults.
//
// Normally, this file is optional (https://reactrouter.com/explanation/special-files#entryclienttsx) and react-router would use the copy in its own package as the default.
// However, when run in Bazel it is not able to find the default copy at runtime.
// TODO: Figure out how to make this work in Bazel.
import { startTransition, StrictMode } from "react";
import { hydrateRoot } from "react-dom/client";
import { HydratedRouter } from "react-router/dom";

startTransition(() => {
  hydrateRoot(
    document,
    <StrictMode>
      <HydratedRouter />
    </StrictMode>
  );
});
