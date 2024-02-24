import { Suspense, memo, useMemo } from "react";
import { graphql } from "relay-runtime";
import {
  UserPageQuery,
  UserPageQuery$data,
} from "$schemas/UserPageQuery.graphql";
import { usePreloadedRoute } from "$util/usePreloadRoute";
import { PostList } from "$domain/posts/PostList";
import { usePaginationFragment } from "react-relay";
import { UserPageRefetchQuery } from "$schemas/UserPageRefetchQuery.graphql";
import { UserPage_user$key } from "$schemas/UserPage_user.graphql";
import { Button } from "@mui/joy";

export const userPageQuery = graphql`
  query UserPageQuery($id: ID!) {
    user(id: $id) {
      ...UserPage_user
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

  const posts = useMemo(
    () => data.posts.edges.map((edge) => edge.node),
    [data]
  );

  return (
    <>
      <PostList fragmentKey={posts} />
      <Button onClick={() => loadPrevious(5)}>More!</Button>
    </>
  );
}

function _UserPageWithSuspense() {
  return (
    <Suspense fallback={"loading"}>
      <_UserPage />
    </Suspense>
  );
}

export const UserPage = memo(_UserPageWithSuspense);
