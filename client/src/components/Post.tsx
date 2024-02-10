import { Avatar, AvatarProps, Card, CardContent, Typography } from "@mui/joy";
import { ReactNode, memo } from "react";

function _Post(props: {
  avatarProps: AvatarProps;
  userName: string;
  message: string;
  children?: ReactNode;
}) {
  return (
    <Card>
      <CardContent orientation={"horizontal"}>
        <Avatar {...props.avatarProps} />
        <Typography>{props.userName}</Typography>
      </CardContent>

      <CardContent>
        <Typography>{props.message}</Typography>
      </CardContent>

      {props.children && <CardContent>{props.children}</CardContent>}
    </Card>
  );
}

export const Post = memo(_Post);
