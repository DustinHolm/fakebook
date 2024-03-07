import {
  Environment,
  Network,
  RecordSource,
  Store,
  FetchFunction,
  SubscribeFunction,
  Observable,
} from "relay-runtime";
import { env } from "./env";
import { createClient } from "graphql-ws";

const fetchFn: FetchFunction = async (request, variables) => {
  const resp = await fetch(env.serverUrl, {
    method: "POST",
    headers: {
      Accept:
        "application/graphql-response+json; charset=utf-8, application/json; charset=utf-8",
      "Content-Type": "application/json",
      // <-- Additional headers like 'Authorization' would go here
    },
    body: JSON.stringify({
      query: request.text, // <-- The GraphQL document composed by Relay
      variables,
    }),
  });

  return await resp.json();
};

const subscribeClient = createClient({
  url: env.serverWsUrl,
});

const subscribeFn: SubscribeFunction = (request, variables) => {
  return Observable.create((sink) => {
    if (!request.text) {
      return sink.error(new Error("Operation text cannot be empty"));
    }

    return subscribeClient.subscribe(
      {
        operationName: request.name,
        query: request.text,
        variables,
      },
      // @ts-expect-error "null" and "undefined" will be handled the same. I hope...
      sink
    );
  });
};

function createRelayEnvironment() {
  return new Environment({
    network: Network.create(fetchFn, subscribeFn),
    store: new Store(new RecordSource()),
  });
}

export const relayEnvironment = createRelayEnvironment();
