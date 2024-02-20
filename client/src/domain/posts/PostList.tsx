import { memo, useMemo } from "react";
import { graphql } from "relay-runtime";
import { useFragment } from "react-relay";
import { PostList_post$key } from "$schemas/PostList_post.graphql";
import { Stack } from "@mui/joy";
import { Post } from "$components/Post";
import { DateTime } from "$util/DateTime";
import { compareDesc } from "date-fns";

const PostList_post = graphql`
  fragment PostList_post on Post @relay(plural: true) {
    id
    createdOn
    content
    author {
      firstName
      lastName
    }
  }
`;

type PostListProps = {
  fragmentKey: PostList_post$key;
};

function _PostList(props: PostListProps) {
  const data = useFragment(PostList_post, props.fragmentKey);
  const posts = useMemo(() => {
    const posts = data.map((post) => ({
      ...post,
      createdOn: DateTime.parse(post.createdOn),
    }));
    posts.sort((a, b) => compareDesc(a.createdOn, b.createdOn));
    return posts;
  }, [data]);

  return (
    <Stack gap={2}>
      {posts.map((post) => (
        <Post
          key={post.id}
          user={post.author}
          createdOn={post.createdOn}
          message={post.content}
        />
      ))}
    </Stack>
  );
}

export const PostList = memo(_PostList);
