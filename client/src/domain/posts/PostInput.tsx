import { PostFrame } from "$components/PostFrame";
import { PostInput_user$key } from "$schemas/PostInput_user.graphql";
import { SubmitData, submit } from "$util/submit";
import { useCurrentTime } from "$util/useCurrentTime";
import { Button, Input } from "@mui/joy";
import { memo } from "react";
import { useFragment, useMutation } from "react-relay";
import { ConnectionHandler, graphql } from "relay-runtime";

const PostInputMutation = graphql`
  mutation PostInputMutation(
    $userId: ID!
    $content: String!
    $connections: [ID!]!
  ) {
    createPost(post: { author: $userId, content: $content })
      @prependEdge(connections: $connections) {
      node {
        ...PostList_post
      }
    }
  }
`;

const PostInput_user = graphql`
  fragment PostInput_user on AppUser {
    id
    firstName
    lastName
  }
`;

type PostInputProps = {
  fragmentKey: PostInput_user$key;
};

function _PostInput(props: PostInputProps) {
  const user = useFragment(PostInput_user, props.fragmentKey);
  const currentTime = useCurrentTime("minutes");
  const [commit, inFlight] = useMutation(PostInputMutation);

  function handleSubmit(data: SubmitData) {
    const connectionID = ConnectionHandler.getConnectionID(
      user.id,
      "HomePageQuery_posts"
    );

    const content = data["content"];

    commit({
      variables: { userId: user.id, content, connections: [connectionID] },
    });
  }

  return (
    <PostFrame user={user} displayDate={currentTime}>
      <form onSubmit={(e) => submit(e, handleSubmit)}>
        <Input
          name="content"
          required
          placeholder="Write something nice!"
          disabled={inFlight}
          endDecorator={<Button type="submit">Post</Button>}
        />
      </form>
    </PostFrame>
  );
}

export const PostInput = memo(_PostInput);
