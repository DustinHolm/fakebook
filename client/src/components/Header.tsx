import { Box, Stack } from "@mui/joy";
import { memo } from "react";

type HeaderProps = { height: string };

function _Header(props: HeaderProps) {
  return (
    <Stack
      justifyContent={"space-between"}
      direction={"row"}
      sx={{
        height: props.height,
        position: "fixed",
        top: 0,
        width: "100%",
        boxSizing: "border-box",
      }}
    >
      <Box>Left</Box>
      <Box>Right</Box>
    </Stack>
  );
}

export const Header = memo(_Header);
