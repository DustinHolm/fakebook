import { memo, useMemo } from "react";
import { ConnectionHandler, graphql } from "relay-runtime";
import { usePreloadedRoute } from "$util/usePreloadRoute";
import { HomePageQuery } from "$schemas/HomePageQuery.graphql";
import { PostList } from "$domain/posts/PostList";
import { Button, Divider, Stack } from "@mui/joy";
import { PostInput } from "$domain/posts/PostInput";
import { usePaginationFragment, useSubscription } from "react-relay";
import { HomePageRefetchQuery } from "$schemas/HomePageRefetchQuery.graphql";
import { HomePage_viewer$key } from "$schemas/HomePage_viewer.graphql";

export const homePageQuery = graphql`
  query HomePageQuery {
    viewer {
      ...PostInput_user
      ...HomePage_viewer
    }
  }
`;

const homePageSubscription = graphql`
  subscription HomePageSubscription($connections: [ID!]!) {
    homeFeed @prependEdge(connections: $connections) {
      node {
        ...PostList_post
      }
    }
  }
`;

const HomePage_homePage = graphql`
  fragment HomePage_viewer on Viewer
  @argumentDefinitions(
    cursor: { type: "String" }
    count: { type: "Int", defaultValue: 5 }
  )
  @refetchable(queryName: "HomePageRefetchQuery") {
    __id
    relevantPosts(before: $cursor, last: $count)
      @connection(key: "HomePage_relevantPosts") {
      edges {
        node {
          ...PostList_post
        }
      }
    }
  }
`;

function _HomePage() {
  const { viewer } = usePreloadedRoute<HomePageQuery>(homePageQuery);

  const { data, loadPrevious } = usePaginationFragment<
    HomePageRefetchQuery,
    HomePage_viewer$key
  >(HomePage_homePage, viewer);

  useSubscription(
    useMemo(
      () => ({
        variables: {
          connections: [
            ConnectionHandler.getConnectionID(
              data.__id,
              "HomePage_relevantPosts"
            ),
          ],
        },
        subscription: homePageSubscription,
      }),
      [data.__id]
    )
  );

  const posts = useMemo(
    () => data.relevantPosts.edges.map((edge) => edge.node),
    [data]
  );

  return (
    <Stack divider={<Divider />} spacing={4}>
      <PostInput fragmentKey={viewer} />
      <PostList fragmentKey={posts} />
      <Button onClick={() => loadPrevious(5)}>More!</Button>
    </Stack>
  );
}

export const HomePage = memo(_HomePage);
