import { Link, Stack } from "@mui/joy";
import { memo } from "react";

function _Header() {
  return (
    <Stack
      justifyContent={"space-between"}
      alignItems={"center"}
      direction={"row"}
      sx={{ minHeight: "64px", padding: 1, backgroundColor: "primary.solidBg" }}
    >
      <Link textColor={"common.white"} level="h1" href={"/"}>
        Fakebook
      </Link>
    </Stack>
  );
}

export const Header = memo(_Header);
