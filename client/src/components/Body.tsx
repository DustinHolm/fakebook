import { ReactNode, memo } from "react";
import { Box, Stack } from "@mui/joy";
import { Header } from "$components/Header";
import { Ad } from "./Ad";

type BodyProps = { children: ReactNode };

function _Body(props: BodyProps) {
  return (
    <Stack direction={"column"} sx={{ height: "100svh" }}>
      <Header />

      <Stack
        direction={"row"}
        sx={{
          overflow: "auto",
          backgroundColor: (t) => t.palette.background.level1,
        }}
      >
        <Ad />

        <Box
          sx={{
            margin: "auto",
            maxWidth: "1000px",
            backgroundColor: "background.body",
            paddingX: "50px",
            paddingY: "10px",
            flexGrow: 1,
          }}
        >
          {props.children}
        </Box>

        <Ad />
      </Stack>
    </Stack>
  );
}

export const Body = memo(_Body);
