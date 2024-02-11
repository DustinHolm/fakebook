import { ReactNode, memo } from "react";
import { Box, Stack } from "@mui/joy";
import { Header } from "./Header";
import { Footer } from "./Footer";

const HEADER_HEIGHT = "64px";

function _Body(props: { children: ReactNode }) {
  return (
    <Stack direction={"column"} sx={{ height: "100svh" }}>
      <Header height={HEADER_HEIGHT} />
      <Box sx={{ margin: "auto", maxWidth: "1000px" }}>{props.children}</Box>
      <Footer />
    </Stack>
  );
}

export const Body = memo(_Body);
