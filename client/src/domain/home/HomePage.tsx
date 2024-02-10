import { Box } from "@mui/joy";
import { memo } from "react";
import { useLazyLoadQuery } from "react-relay";
import { graphql } from "relay-runtime";
import { Post } from "../../components/Post";
import { HomePageQuery } from "./__generated__/HomePageQuery.graphql";

const postQuery = graphql`
  query HomePageQuery {
    user(id: 1) {
      firstName
      lastName
      posts {
        id
        content
      }
    }
  }
`;

function _HomePage() {
  const { user } = useLazyLoadQuery<HomePageQuery>(postQuery, {});

  return (
    <Box sx={{ backgroundColor: "white" }}>
      {user.posts.map((post) => (
        <Post
          key={post.id}
          avatarProps={{ children: user.firstName[0] + user.lastName[0] }}
          userName={user.firstName + " " + user.lastName}
          message={post.content}
        />
      ))}
    </Box>
  );
}

export const HomePage = memo(_HomePage);
