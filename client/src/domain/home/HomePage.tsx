import { Box } from "@mui/joy";
import { memo, useMemo } from "react";
import { graphql } from "relay-runtime";
import { Post } from "../../components/Post";
import { HomePageQuery } from "./__generated__/HomePageQuery.graphql";
import { DateTime } from "../date_time/DateTime";
import { usePreloadedRoute } from "../../util/usePreloadRoute";

export const homePageQuery = graphql`
  query HomePageQuery($id: ID!) {
    user(id: $id) {
      friends {
        posts {
          pid
          author {
            firstName
            lastName
          }
          createdOn
          content
        }
      }
    }
  }
`;

function _HomePage() {
  const { user } = usePreloadedRoute<HomePageQuery>(homePageQuery);

  const posts = useMemo(() => {
    const posts = user.friends.flatMap((friend) => friend.posts);
    posts.sort((a, b) => b.createdOn.localeCompare(a.createdOn));
    return posts;
  }, [user]);

  return (
    <Box sx={{ backgroundColor: "white" }}>
      {posts.map((post) => (
        <Post
          key={post.pid}
          user={post.author}
          createdOn={DateTime.parse(post.createdOn)}
          message={post.content}
        />
      ))}
    </Box>
  );
}

export const HomePage = memo(_HomePage);
