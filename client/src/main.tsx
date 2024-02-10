import { RelayEnvironmentProvider } from "react-relay";
import { relayEnvironment } from "./relayEnvironment";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router-dom";
import { router } from "./routing/router";
import { StrictMode } from "react";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <RelayEnvironmentProvider environment={relayEnvironment}>
    <StrictMode>
      <RouterProvider router={router} />
    </StrictMode>
  </RelayEnvironmentProvider>
);
