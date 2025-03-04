import { Avatar, Card, CardContent, Stack, Typography } from "@mui/joy";
import { ReactNode, memo } from "react";
import { DateTime } from "$util/DateTime";

type PostFrameProps = {
  user: { id?: string; firstName: string; lastName: string };
  displayDate: Date;
  children: ReactNode;
};

function _PostFrame(props: PostFrameProps) {
  const idGiven = props.user.id !== undefined;

  return (
    <Card>
      <Stack direction={"row"} justifyContent={"space-between"}>
        <Stack direction={"row"} spacing={1} alignItems={"center"}>
          <Avatar
            component={idGiven ? "a" : "div"}
            href={idGiven ? `/user/${props.user.id}` : undefined}
            sx={{ textDecoration: "none" }}
          >
            {props.user.firstName[0] + props.user.lastName[0]}
          </Avatar>

          <Typography>
            {props.user.firstName + " " + props.user.lastName}
          </Typography>
        </Stack>

        <Typography>{DateTime.format(props.displayDate)}</Typography>
      </Stack>

      <CardContent>{props.children}</CardContent>
    </Card>
  );
}

export const PostFrame = memo(_PostFrame);
