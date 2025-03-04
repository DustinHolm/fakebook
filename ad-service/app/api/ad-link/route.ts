import prisma from "$/lib/prisma";
import { NextRequest, NextResponse } from "next/server";

export const GET = async (req: NextRequest) => {
  const host = req.headers.get("Host");
  if (!host) return new NextResponse(null, { status: 400 });

  console.log(req.headers);

  // This will someday be some convoluted logic based on user interests and whatnot...
  const ads = await prisma.ad.findMany({ select: { pid: true } });
  const selection = Math.floor(Math.random() * ads.length);
  const selectedAd = ads[selection];
  if (!selectedAd) return new NextResponse(null, { status: 404 });

  return NextResponse.json({
    adLink: `http://${host}/embed/${selectedAd.pid}`,
  });
};
