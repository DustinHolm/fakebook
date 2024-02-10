import { Box, Stack } from "@mui/joy";
import { memo } from "react";

function Header() {
  return (
    <Stack justifyContent={"space-between"} direction={"row"}>
      <Box>Left</Box>
      <Box>Right</Box>
    </Stack>
  );
}

export default memo(Header);
