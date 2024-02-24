import { ReactNode, memo } from "react";
import { Box, Stack } from "@mui/joy";
import { Header } from "$components/Header";
import { Footer } from "$components/Footer";

type BodyProps = { children: ReactNode };

function _Body(props: BodyProps) {
  return (
    <Stack direction={"column"} sx={{ height: "100svh" }} spacing={1}>
      <Header />

      <Box sx={{ overflow: "auto", backgroundColor: "background.body" }}>
        <Box sx={{ margin: "auto", maxWidth: "1000px" }}>{props.children}</Box>
      </Box>

      <Footer />
    </Stack>
  );
}

export const Body = memo(_Body);
