import { Stack, Typography } from "@mui/joy";
import { memo } from "react";
import { useRouteError } from "react-router";

function _ErrorFallback() {
  const error = useRouteError();
  const message =
    (error as { message: string | undefined })?.message ?? "No error message.";

  return (
    <Stack
      margin={"auto"}
      alignItems={"center"}
      justifyContent={"space-around"}
    >
      <Typography>Failed spectacularly</Typography>
      <Typography>{message}</Typography>
    </Stack>
  );
}

export const ErrorFallback = memo(_ErrorFallback);
