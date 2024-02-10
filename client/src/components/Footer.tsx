import { Stack, Box } from "@mui/joy";
import { memo } from "react";

function _Footer() {
  return (
    <Stack justifyContent={"space-between"} direction={"row"}>
      <Box>Left</Box>
      <Box>Right</Box>
    </Stack>
  );
}

export const Footer = memo(_Footer);
