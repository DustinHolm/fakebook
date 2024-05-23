import Heading from "$/components/Heading";
import prisma from "$/lib/prisma";
import Link from "next/link";
import { FC } from "react";

export const dynamic = "force-dynamic";

const Page: FC = async () => {
  const ads = await prisma.ad.findMany({
    select: {
      pid: true,
      title: true,
      _count: { select: { interactions: true } },
    },
    orderBy: { interactions: { _count: "desc" } },
  });

  return (
    <>
      <Heading>All ads ordered by hotness!</Heading>

      <ul className="w-1/3 mx-auto">
        {ads.map((ad) => (
          <li key={ad.pid}>
            <Link
              href={`/editor/ad/${ad.pid}`}
              className="hover:underline hover:decoration-teal-400"
            >
              {ad.title}
            </Link>
            <span className="text-gray-500 ml-4">
              Hotness: {ad._count.interactions}
            </span>
          </li>
        ))}
      </ul>
    </>
  );
};

export default Page;
