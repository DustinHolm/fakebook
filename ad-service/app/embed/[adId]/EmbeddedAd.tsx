"use client";
import { Ad } from "@prisma/client";
import { FC, memo } from "react";

type Props = {
  ad: Ad;
};

const EmbeddedAd: FC<Props> = (props) => (
  <main
    className="w-80 h-60 cursor-pointer"
    onClick={() => {
      fetch(`/api/ad-click/${props.ad.pid}`, { method: "post" });
    }}
  >
    <h1 className="text-lg">{props.ad.title}</h1>
    <p>{props.ad.details}</p>
  </main>
);

export default memo(EmbeddedAd);
