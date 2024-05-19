/*
  Warnings:

  - You are about to drop the column `link` on the `Ad` table. All the data in the column will be lost.
  - Added the required column `interactionType` to the `AdInteraction` table without a default value. This is not possible if the table is not empty.

*/
-- CreateEnum
CREATE TYPE "InteractionType" AS ENUM ('SERVED', 'CLICKED');

-- AlterTable
ALTER TABLE "Ad" DROP COLUMN "link";

-- AlterTable
ALTER TABLE "AdInteraction" ADD COLUMN     "interactionType" "InteractionType" NOT NULL;
