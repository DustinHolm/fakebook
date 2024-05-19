-- CreateTable
CREATE TABLE "Ad" (
    "pid" SERIAL NOT NULL,
    "title" TEXT NOT NULL,
    "details" TEXT NOT NULL,
    "link" TEXT NOT NULL,

    CONSTRAINT "Ad_pkey" PRIMARY KEY ("pid")
);

-- CreateTable
CREATE TABLE "AdInteraction" (
    "pid" SERIAL NOT NULL,
    "adId" INTEGER NOT NULL,
    "userId" TEXT NOT NULL,
    "time" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "AdInteraction_pkey" PRIMARY KEY ("pid")
);

-- AddForeignKey
ALTER TABLE "AdInteraction" ADD CONSTRAINT "AdInteraction_adId_fkey" FOREIGN KEY ("adId") REFERENCES "Ad"("pid") ON DELETE RESTRICT ON UPDATE CASCADE;
