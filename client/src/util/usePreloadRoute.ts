import { useEffect } from "react";
import {
  GraphQLTaggedNode,
  PreloadedQuery,
  usePreloadedQuery,
} from "react-relay";
import { useLoaderData, useNavigation } from "react-router";
import { OperationType } from "relay-runtime";

export function usePreloadedRoute<T extends OperationType>(
  query: GraphQLTaggedNode
) {
  const navigation = useNavigation();
  const data = useLoaderData() as PreloadedQuery<T>;

  useEffect(
    () => () => {
      // May cause problems with nested routes. Keep an eye on the console.
      if (navigation.state === "loading") {
        data.dispose();
      }
    },
    [data, navigation]
  );

  return usePreloadedQuery<T>(query, data);
}
