import { AdQuery } from "$schemas/AdQuery.graphql";
import { Box } from "@mui/joy";
import { memo, Suspense } from "react";
import { useLazyLoadQuery } from "react-relay";
import { graphql } from "relay-runtime";

const adQuery = graphql`
  query AdQuery {
    viewer {
      relevantAdUrl
    }
  }
`;

function _Ad() {
  const { viewer } = useLazyLoadQuery<AdQuery>(adQuery, {});

  return (
    <iframe style={{ border: "1px black solid" }} src={viewer.relevantAdUrl} />
  );
}

function _AdWithSuspense() {
  return (
    <Box
      sx={{
        position: "sticky",
        top: "0px",
        paddingTop: "10svh",
        paddingX: "10px",
        flexGrow: 0,
      }}
    >
      <Suspense fallback="Loading...">
        <_Ad />
      </Suspense>
    </Box>
  );
}

export const Ad = memo(_AdWithSuspense);
