import { Suspense, memo, useMemo } from "react";
import { graphql } from "relay-runtime";
import { UserPageQuery } from "$schemas/UserPageQuery.graphql";
import { usePreloadedRoute } from "$util/usePreloadRoute";
import { PostList } from "$domain/posts/PostList";

export const userPageQuery = graphql`
  query UserPageQuery($id: ID!) {
    user(id: $id) {
      posts(first: 100) {
        edges {
          node {
            ...PostList_post
          }
        }
      }
    }
  }
`;

function _UserPage() {
  const { user } = usePreloadedRoute<UserPageQuery>(userPageQuery);

  const posts = useMemo(
    () => user.posts.edges.map((edge) => edge.node),
    [user]
  );

  return <PostList fragmentKey={posts} />;
}

function _UserPageWithSuspense() {
  return (
    <Suspense fallback={"loading"}>
      <_UserPage />
    </Suspense>
  );
}

export const UserPage = memo(_UserPageWithSuspense);
