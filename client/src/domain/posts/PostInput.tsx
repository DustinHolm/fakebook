import { FormInput } from "$components/FormInput";
import { PostFrame } from "$components/PostFrame";
import { PostInput_user$key } from "$schemas/PostInput_user.graphql";
import { useCurrentTime } from "$util/useCurrentTime";
import { Button, Stack } from "@mui/joy";
import { memo, useCallback } from "react";
import { useForm } from "react-hook-form";
import { useFragment, useMutation } from "react-relay";
import { ConnectionHandler, graphql } from "relay-runtime";

const PostInputMutation = graphql`
  mutation PostInputMutation($content: String!, $connections: [ID!]!) {
    createPost(input: { content: $content })
      @prependEdge(connections: $connections) {
      node {
        ...PostList_post
      }
    }
  }
`;

const PostInput_user = graphql`
  fragment PostInput_user on Viewer {
    __id
    firstName
    lastName
  }
`;

type PostInputProps = {
  fragmentKey: PostInput_user$key;
};

type FormInput = {
  content: string;
};

function _PostInput(props: PostInputProps) {
  const user = useFragment(PostInput_user, props.fragmentKey);
  const currentTime = useCurrentTime("minutes");
  const [commit, inFlight] = useMutation(PostInputMutation);
  const form = useForm<FormInput>();

  const handleSubmit = useCallback(
    function (data: FormInput) {
      const connectionID = ConnectionHandler.getConnectionID(
        user.__id,
        "HomePage_relevantPosts"
      );

      commit({
        variables: {
          content: data.content,
          connections: [connectionID],
        },
      });

      form.reset();
    },
    [user.__id, commit, form]
  );

  return (
    <PostFrame user={user} displayDate={currentTime}>
      {/* eslint-disable-next-line @typescript-eslint/no-misused-promises
          -- different behaviour! */}
      <form onSubmit={form.handleSubmit(handleSubmit)}>
        <Stack direction={"row"} spacing={1} justifyContent={"space-between"}>
          <FormInput
            registerProps={form.register("content", {
              required: "Please write something here",
            })}
            placeholder="Write something nice!"
            disabled={inFlight}
            error={form.formState.errors.content?.message}
            sx={{ flexGrow: 1 }}
          />

          <Button type="submit">Post</Button>
        </Stack>
      </form>
    </PostFrame>
  );
}

export const PostInput = memo(_PostInput);
