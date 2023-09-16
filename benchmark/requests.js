import http from "k6/http";
import { graphqlUrl } from "./urls.js";

export const User = (id) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `query User($id: Int) {
                user(id: $id) {
                  userId
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
      query: `query UserFriends($id: Int) {
                user(id: $id) {
                  userId
                  firstName
                  lastName
                  friends {
                    userId
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

export const UserThriceNestedFriends = (id) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `query UserThriceNestedFriends($id: Int) {
                user(id: $id) {
                  userId
                  firstName
                  lastName
                  friends {
                    userId
                    firstName
                    lastName
                    friends {
                      userId
                      firstName
                      lastName
                      friends {
                        userId
                        firstName
                        lastName
                        friends {
                          userId
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
