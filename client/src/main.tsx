import { RelayEnvironmentProvider } from "react-relay";
import { relayEnvironment } from "./relayEnvironment";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router-dom";
import { router } from "./routing/router";
import { StrictMode } from "react";
import { CssBaseline, CssVarsProvider } from "@mui/joy";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <RelayEnvironmentProvider environment={relayEnvironment}>
    <StrictMode>
      <CssVarsProvider>
        <CssBaseline />

        <RouterProvider router={router} />
      </CssVarsProvider>
    </StrictMode>
  </RelayEnvironmentProvider>
);
