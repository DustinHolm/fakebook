import {
  Avatar,
  AvatarProps,
  Button,
  Card,
  CardContent,
  Typography,
} from "@mui/joy";
import { ReactNode, memo } from "react";

function Post(props: {
  avatarProps: AvatarProps;
  userName: string;
  message: string;
  children: ReactNode;
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

      <CardContent>{props.children}</CardContent>
    </Card>
  );
}

export default memo(Post);
