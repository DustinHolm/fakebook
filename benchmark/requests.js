import http from "k6/http";
import { graphqlUrl } from "./urls.js";

export const User = (id) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `query User($id: ID!) {
                user(id: $id) {
                  id
                  firstName
                  lastName
                }
              }`,
      variables: {
        id: id,
      },
    })
  );

export const UserFriends = (id) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `query UserFriends($id: ID!) {
                user(id: $id) {
                  id
                  firstName
                  lastName
                  friends {
                    id
                    firstName
                    lastName
                  }
                }
              }`,
      variables: {
        id: id,
      },
    })
  );

export const UserFriendsPosts = (id) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `query UserFriendsPosts($id: ID!) {
                user(id: $id) {
                  id
                  firstName
                  lastName
                  friends {
                    id
                    firstName
                    lastName
                    posts {
                      content
                    }
                  }
                }
              }`,
      variables: {
        id: id,
      },
    })
  );

export const UserThriceNestedFriends = (id) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `query UserThriceNestedFriends($id: ID!) {
                user(id: $id) {
                  id
                  firstName
                  lastName
                  friends {
                    id
                    firstName
                    lastName
                    friends {
                      id
                      firstName
                      lastName
                      friends {
                        id
                        firstName
                        lastName
                        friends {
                          id
                          firstName
                          lastName
                        }
                      }
                    }
                  }
                }
              }`,
      variables: {
        id: id,
      },
    })
  );
