import Heading from "$/components/Heading";
import prisma from "$/lib/prisma";
import { FC } from "react";
import InteractionTable from "./InteractionTable";

type Props = {
  params: { adId: string };
};

const Page: FC<Props> = async ({ params }) => {
  const ad = await prisma.ad.findUniqueOrThrow({
    where: { pid: Number.parseInt(params.adId) },
    include: {
      interactions: { orderBy: { time: "desc" } },
    },
  });

  return (
    <>
      <Heading>Nice ad you got there!</Heading>

      <div className="flex-col space-y-8">
        <div className="flex-col space-y-2">
          <h2 className="text-2xl">Metadata</h2>

          <p>Title: {ad.title}</p>

          <p>Description: {ad.details}</p>
        </div>

        <div className="flex-col space-y-2">
          <h2 className="text-2xl">Interactions</h2>

          {ad.interactions.length > 0 ? (
            <InteractionTable interactions={ad.interactions} />
          ) : (
            <p>No interactions yet. What a shame!</p>
          )}
        </div>
      </div>
    </>
  );
};

export default Page;
