"use client";
import { Ad } from "@prisma/client";
import { FC, memo } from "react";

type Props = {
  ad: Ad;
};

const EmbeddedAd: FC<Props> = (props) => (
  <a
    className="w-full h-full cursor-pointer block"
    onClick={async () => {
      void fetch(`/api/ad-click/${props.ad.pid}`, { method: "post" });
    }}
    href={`/embed/${props.ad.pid}`}
    target={"_blank"}
  >
    <h1 className="text-lg">{props.ad.title}</h1>
    <p>{props.ad.details}</p>
  </a>
);

export default memo(EmbeddedAd);
