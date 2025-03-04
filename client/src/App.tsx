import { CssVarsProvider, CssBaseline } from "@mui/joy";
import { memo, StrictMode } from "react";
import { RelayEnvironmentProvider } from "react-relay";
import { RouterProvider } from "react-router";
import { relayEnvironment } from "./relayEnvironment";
import { router } from "./routing/router";

function _App() {
  return (
    <RelayEnvironmentProvider environment={relayEnvironment}>
      <StrictMode>
        <CssVarsProvider>
          <CssBaseline />

          <RouterProvider router={router} />
        </CssVarsProvider>
      </StrictMode>
    </RelayEnvironmentProvider>
  );
}

export const App = memo(_App);
