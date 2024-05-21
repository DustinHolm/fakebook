import prisma from "$/lib/prisma";
import { InteractionType } from "@prisma/client";
import { NextRequest, NextResponse } from "next/server";
import { z } from "zod";

const schema = z.number().int().positive().safe();

export const POST = async (
  _: NextRequest,
  { params }: { params: { adId: string } }
) => {
  const pid = Number.parseInt(params.adId);
  if (!schema.safeParse(pid).success)
    return new NextResponse(null, { status: 400 });

  await prisma.adInteraction
    .create({
      data: {
        interactionType: InteractionType.CLICKED,
        ad: { connect: { pid } },
      },
    })
    .then(() => {
      console.debug("Clicked ad:", pid);
    });

  return new NextResponse();
};
