import { memo, useMemo } from "react";
import { ConnectionHandler, graphql } from "relay-runtime";
import { UserPageQuery } from "$schemas/UserPageQuery.graphql";
import { usePreloadedRoute } from "$util/usePreloadRoute";
import { PostList } from "$domain/posts/PostList";
import { usePaginationFragment, useSubscription } from "react-relay";
import { UserPageRefetchQuery } from "$schemas/UserPageRefetchQuery.graphql";
import { UserPage_user$key } from "$schemas/UserPage_user.graphql";
import { Button, Divider, Stack } from "@mui/joy";

export const userPageQuery = graphql`
  query UserPageQuery($id: ID!) {
    user(id: $id) {
      ...UserPage_user
    }
  }
`;

const userPageSubscription = graphql`
  subscription UserPageSubscription($userId: ID!, $connections: [ID!]!) {
    userFeed(userId: $userId) @prependEdge(connections: $connections) {
      node {
        ...PostList_post
      }
    }
  }
`;

const UserPage_user = graphql`
  fragment UserPage_user on AppUser
  @argumentDefinitions(
    cursor: { type: "String" }
    count: { type: "Int", defaultValue: 5 }
  )
  @refetchable(queryName: "UserPageRefetchQuery") {
    id
    posts(before: $cursor, last: $count)
      @connection(key: "UserPage_user_posts") {
      edges {
        node {
          ...PostList_post
        }
      }
    }
  }
`;

function _UserPage() {
  const { user } = usePreloadedRoute<UserPageQuery>(userPageQuery);

  const { data, loadPrevious } = usePaginationFragment<
    UserPageRefetchQuery,
    UserPage_user$key
  >(UserPage_user, user);

  useSubscription(
    useMemo(
      () => ({
        variables: {
          userId: data.id,
          connections: [
            ConnectionHandler.getConnectionID(data.id, "UserPage_user_posts"),
          ],
        },
        subscription: userPageSubscription,
      }),
      [data.id]
    )
  );

  const posts = useMemo(
    () => data.posts.edges.map((edge) => edge.node),
    [data]
  );

  return (
    <Stack divider={<Divider />} spacing={4}>
      <PostList fragmentKey={posts} />
      <Button onClick={() => loadPrevious(5)}>More!</Button>
    </Stack>
  );
}

export const UserPage = memo(_UserPage);
