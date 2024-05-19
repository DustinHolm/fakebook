import type { Metadata } from "next";
import "./globals.css";
import NavBar from "$/components/NavBar";

export const metadata: Metadata = {
  title: "Ad Service",
  description:
    "Who does not like a nice ad or two? Or three? Or maybe even more?",
};

const addresses = [
  { name: "Home", href: "/" },
  { name: "Browse", href: "/overview" },
  { name: "Create", href: "/create" },
];

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="bg-white text:black dark:bg-black dark:text-white">
        <header>
          <NavBar addresses={addresses} />
        </header>

        <main className="my-16 mx-auto max-w-screen-lg">{children}</main>
      </body>
    </html>
  );
}
