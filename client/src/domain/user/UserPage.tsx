import { Suspense, memo } from "react";
import { graphql } from "relay-runtime";
import { UserPageQuery } from "$schemas/UserPageQuery.graphql";
import { usePreloadedRoute } from "$util/usePreloadRoute";
import { PostList } from "$domain/posts/PostList";

export const userPageQuery = graphql`
  query UserPageQuery($id: ID!) {
    user(id: $id) {
      posts {
        ...PostList_post
      }
    }
  }
`;

function _UserPage() {
  const { user } = usePreloadedRoute<UserPageQuery>(userPageQuery);

  return <PostList data={user.posts} />;
}

function _UserPageWithSuspense() {
  return (
    <Suspense fallback={"loading"}>
      <_UserPage />
    </Suspense>
  );
}

export const UserPage = memo(_UserPageWithSuspense);
