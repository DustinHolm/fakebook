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
        id,
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
        id,
      },
    })
  );

export const UserFriendsPosts = (id, nPaginated) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `query UserFriendsPosts($id: ID!, $nPaginated: Int) {
                user(id: $id) {
                  id
                  firstName
                  lastName
                  friends {
                    id
                    firstName
                    lastName
                    posts(first: $nPaginated) {
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
        id,
        nPaginated,
      },
    })
  );

export const UserFriendsPostsComments = (id, nPaginated) =>
  http.post(
    graphqlUrl,
    JSON.stringify({
      query: `query UserFriendsPostsComments($id: ID!, $nPaginated: Int) {
                user(id: $id) {
                  id
                  firstName
                  lastName
                  friends {
                    id
                    firstName
                    lastName
                    posts(first: $nPaginated) {
                      edges {
                        node {
                          comments(first: $nPaginated) {
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
        id,
        nPaginated,
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
        id,
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
        first,
        last,
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
        user,
        friend,
      },
    })
  );
