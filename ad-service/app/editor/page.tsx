import Heading from "$/components/Heading";
import Link from "next/link";
import { FC } from "react";

const Page: FC = () => {
  return (
    <>
      <Heading>Hello! You seem to like ads.</Heading>

      <p className="text-xl text-center">
        Maybe you would like to{" "}
        <Link href={"/editor/overview"} className="text-blue-400">
          browse own exquisite selection of the worlds finest ads
        </Link>
        . Or maybe you would prefer to{" "}
        <Link href={"/editor/create"} className="text-blue-400">
          create your own ad
        </Link>
        ?
      </p>
    </>
  );
};

export default Page;
