import prisma from "$/lib/prisma";
import { InteractionType } from "@prisma/client";
import { FC } from "react";
import EmbeddedAd from "./EmbeddedAd";

export const dynamic = "force-dynamic";

type Props = {
  params: { adId: string };
};

const Page: FC<Props> = async ({ params }) => {
  const ad = await prisma.ad.findUniqueOrThrow({
    where: { pid: Number.parseInt(params.adId) },
  });

  prisma.adInteraction
    .create({
      data: {
        interactionType: InteractionType.SERVED,
        ad: { connect: { pid: ad.pid } },
      },
    })
    .then(() => {
      console.debug("Served ad:", ad.pid);
    });

  return <EmbeddedAd ad={ad} />;
};

export default Page;
