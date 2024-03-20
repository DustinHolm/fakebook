import { createBrowserRouter } from "react-router-dom";
import { BodyPage } from "$domain/body/BodyPage";
import { HomePage, homePageQuery } from "$domain/home/HomePage";
import { UserPage, userPageQuery } from "$domain/user/UserPage";
import { loadQuery } from "react-relay";
import { relayEnvironment } from "../relayEnvironment";
import { ErrorFallback } from "./ErrorFallback";

export const router = createBrowserRouter([
  {
    element: <BodyPage />,
    children: [
      {
        path: "/",
        element: <HomePage />,
        loader: async () => {
          return loadQuery(relayEnvironment, homePageQuery, {});
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
