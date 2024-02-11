import { Avatar, Card, CardContent, Stack, Typography } from "@mui/joy";
import { ReactNode, memo } from "react";
import { DateTime } from "../domain/date_time/DateTime";

function _Post(props: {
  user: { firstName: string; lastName: string };
  createdOn: Date;
  message: string;
  children?: ReactNode;
}) {
  return (
    <Card>
      <Stack direction={"row"} justifyContent={"space-between"}>
        <Stack direction={"row"} spacing={1} alignItems={"center"}>
          <Avatar>{props.user.firstName[0] + props.user.lastName[0]}</Avatar>

          <Typography>
            {props.user.firstName + " " + props.user.lastName}
          </Typography>
        </Stack>

        <Typography>{DateTime.format(props.createdOn)}</Typography>
      </Stack>

      <CardContent>
        <Typography>{props.message}</Typography>
      </CardContent>

      {props.children && <CardContent>{props.children}</CardContent>}
    </Card>
  );
}

export const Post = memo(_Post);
