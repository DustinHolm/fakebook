import type { Metadata } from "next";
import "./globals.css";
import { ReactNode } from "react";

export const metadata: Metadata = {
  title: "Ad Service",
  description:
    "Who does not like a nice ad or two? Or three? Or maybe even more?",
};

export default function Layout({
  children,
}: Readonly<{
  children: ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="bg-white text:black dark:bg-black dark:text-white">
        {children}
      </body>
    </html>
  );
}
