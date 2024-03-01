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
                      edges {
                        node {
                          content
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

export const UserFriendsPostsComments = (id) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `query UserFriendsPostsComments($id: ID!) {
                user(id: $id) {
                  id
                  firstName
                  lastName
                  friends {
                    id
                    firstName
                    lastName
                    posts {
                      edges {
                        node {
                          comments {
                            edges {
                              node {
                                content
                              }
                            }
                          }
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

export const CreateUser = (first, last) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `mutation CreateUser($first: String!, $last: String!) {
                createUser(input: { firstName: $first, lastName: $last }) {
                  id
                }
              }`,
      variables: {
        first: first,
        last: last,
      },
    })
  );

export const AddFriend = (user, friend) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `mutation CreateUser($user: ID!, $friend: ID!) {
                addFriend(input: { user: $user, friend: $friend }) {
                  id
                }
              }`,
      variables: {
        user: user,
        friend: friend,
      },
    })
  );
