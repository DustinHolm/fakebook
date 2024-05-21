import type { Metadata } from "next";
import NavBar from "$/components/NavBar";
import { ReactNode } from "react";

export const metadata: Metadata = {
  title: "Ad Service",
  description:
    "Who does not like a nice ad or two? Or three? Or maybe even more?",
};

const addresses = [
  { name: "Home", href: "/editor" },
  { name: "Browse", href: "/editor/overview" },
  { name: "Create", href: "/editor/create" },
];

export default function Layout({
  children,
}: Readonly<{
  children: ReactNode;
}>) {
  return (
    <>
      <header>
        <NavBar addresses={addresses} />
      </header>

      <main className="my-16 mx-auto max-w-screen-lg">{children}</main>
    </>
  );
}
