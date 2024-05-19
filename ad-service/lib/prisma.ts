import { PrismaClient } from "@prisma/client";

declare const globalThis: {
  prisma: PrismaClient;
} & typeof global;

let prisma: PrismaClient;

if (process.env.NODE_ENV === "production") {
  prisma = new PrismaClient();
} else {
  if (!globalThis.prisma) {
    globalThis.prisma = new PrismaClient();
  }
  prisma = globalThis.prisma;
}

export default prisma;
