import { CardContent, Typography } from "@mui/joy";
import { memo } from "react";
import { PostFrame } from "./PostFrame";

type PostProps = {
  user: { firstName: string; lastName: string };
  createdOn: Date;
  message: string;
};

function _Post(props: PostProps) {
  return (
    <PostFrame user={props.user} displayDate={props.createdOn}>
      <CardContent>
        <Typography>{props.message}</Typography>
      </CardContent>
    </PostFrame>
  );
}

export const Post = memo(_Post);
