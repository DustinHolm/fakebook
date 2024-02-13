import { memo, useMemo } from "react";
import { graphql } from "relay-runtime";
import { usePreloadedRoute } from "$util/usePreloadRoute";
import { HomePageQuery } from "$schemas/HomePageQuery.graphql";
import { PostList } from "$domain/posts/PostList";

export const homePageQuery = graphql`
  query HomePageQuery($id: ID!) {
    user(id: $id) {
      posts {
        ...PostList_post
      }
      friends {
        posts {
          ...PostList_post
        }
      }
    }
  }
`;

function _HomePage() {
  const { user } = usePreloadedRoute<HomePageQuery>(homePageQuery);

  const posts = useMemo(
    () => user.friends.flatMap((friend) => friend.posts).concat(user.posts),
    [user]
  );

  return <PostList data={posts} />;
}

export const HomePage = memo(_HomePage);
