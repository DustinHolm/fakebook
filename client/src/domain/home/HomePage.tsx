import { memo, useMemo } from "react";
import { graphql } from "relay-runtime";
import { usePreloadedRoute } from "$util/usePreloadRoute";
import { HomePageQuery } from "$schemas/HomePageQuery.graphql";
import { PostList } from "$domain/posts/PostList";
import { Divider, Stack } from "@mui/joy";
import { PostInput } from "$domain/posts/PostInput";

export const homePageQuery = graphql`
  query HomePageQuery($id: ID!) {
    user(id: $id) {
      ...PostInput_user
      posts(first: 100) @connection(key: "HomePageQuery_posts") {
        edges {
          node {
            ...PostList_post
          }
        }
      }
      friends {
        posts(first: 100) {
          edges {
            node {
              ...PostList_post
            }
          }
        }
      }
    }
  }
`;

function _HomePage() {
  const { user } = usePreloadedRoute<HomePageQuery>(homePageQuery);

  const posts = useMemo(
    () =>
      user.friends
        .flatMap((friend) => friend.posts.edges.map((edge) => edge.node))
        .concat(user.posts.edges.map((edge) => edge.node)),
    [user]
  );

  return (
    <Stack divider={<Divider />} spacing={4}>
      <PostInput fragmentKey={user} />
      <PostList fragmentKey={posts} />
    </Stack>
  );
}

export const HomePage = memo(_HomePage);
