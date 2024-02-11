import { Stack } from "@mui/joy";
import { Suspense, memo } from "react";
import { graphql } from "relay-runtime";
import { Post } from "../../components/Post";
import { UserPageQuery } from "./__generated__/UserPageQuery.graphql";
import { DateTime } from "../date_time/DateTime";
import { usePreloadedRoute } from "../../util/usePreloadRoute";

export const userPageQuery = graphql`
  query UserPageQuery($id: ID!) {
    user(id: $id) {
      firstName
      lastName
      posts {
        pid
        content
        createdOn
      }
    }
  }
`;

function _UserPage() {
  const { user } = usePreloadedRoute<UserPageQuery>(userPageQuery);

  return (
    <Stack gap={2}>
      {user.posts.map((post) => (
        <Post
          key={post.pid}
          user={user}
          message={post.content}
          createdOn={DateTime.parse(post.createdOn)}
        />
      ))}
    </Stack>
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
