import { ReactNode, memo } from "react";
import { Box, Stack } from "@mui/joy";
import { Header } from "$components/Header";
import { Footer } from "$components/Footer";

const HEADER_HEIGHT = "64px";

type BodyProps = { children: ReactNode };

function _Body(props: BodyProps) {
  return (
    <Stack direction={"column"} sx={{ height: "100svh" }}>
      <Header height={HEADER_HEIGHT} />
      <Box sx={{ margin: "auto", maxWidth: "1000px" }}>{props.children}</Box>
      <Footer />
    </Stack>
  );
}

export const Body = memo(_Body);
