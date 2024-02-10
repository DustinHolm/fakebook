import { ReactNode, memo } from "react";
import { Box } from "@mui/joy";
import { Header } from "./Header";
import { Footer } from "./Footer";

function _Body(props: { children: ReactNode }) {
  return (
    <Box sx={{ backgroundColor: "#ee99ee" }}>
      <Header />
      <Box sx={{ margin: "auto", maxWidth: "1000px" }}>{props.children}</Box>
      <Footer />
    </Box>
  );
}

export const Body = memo(_Body);
