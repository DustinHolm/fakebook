import { createBrowserRouter } from "react-router-dom";
import { BodyPage } from "../domain/body/BodyPage";
import { HomePage } from "../domain/home/HomePage";

export const router = createBrowserRouter([
  {
    element: <BodyPage />,
    children: [{ path: "/", element: <HomePage /> }],
  },
]);
