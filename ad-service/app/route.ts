import { NextRequest, NextResponse } from "next/server";

export const GET = (req: NextRequest) => {
  const nextReq = req.nextUrl.clone();
  nextReq.pathname = "/editor";
  return NextResponse.redirect(nextReq, req);
};
