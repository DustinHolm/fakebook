import { createBrowserRouter } from "react-router-dom";
import { BodyPage } from "$domain/body/BodyPage";
import { HomePage, homePageQuery } from "$domain/home/HomePage";
import { UserPage, userPageQuery } from "$domain/user/UserPage";
import { loadQuery } from "react-relay";
import { relayEnvironment } from "../relayEnvironment";
import { ErrorFallback } from "./ErrorFallback";
import { useGlobalState } from "$domain/globalState";

export const router = createBrowserRouter([
  {
    element: <BodyPage />,
    children: [
      {
        path: "/",
        element: <HomePage />,
        loader: async () => {
          const id = useGlobalState.getState().currentUser.id;
          if (!id) {
            throw Error("Not logged in and no way to currently login :(");
          }
          return loadQuery(relayEnvironment, homePageQuery, { id });
        },
      },
      {
        path: "/user/:userId",
        element: <UserPage />,
        loader: async ({ params }) => {
          const id = params.userId;
          if (!id) {
            throw Error("Require userId to be defined!");
          }
          return loadQuery(relayEnvironment, userPageQuery, { id });
        },
      },
    ],
    errorElement: <ErrorFallback />,
  },
]);
