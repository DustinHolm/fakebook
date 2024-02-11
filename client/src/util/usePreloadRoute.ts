import {
  GraphQLTaggedNode,
  PreloadedQuery,
  usePreloadedQuery,
} from "react-relay";
import { useLoaderData } from "react-router";
import { OperationType } from "relay-runtime";

export function usePreloadedRoute<T extends OperationType>(
  query: GraphQLTaggedNode
) {
  const data = useLoaderData();
  return usePreloadedQuery<T>(query, data as PreloadedQuery<T>);
}
