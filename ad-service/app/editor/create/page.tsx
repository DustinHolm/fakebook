import Heading from "$/components/Heading";
import prisma from "$/lib/prisma";
import { redirect } from "next/navigation";
import { FC } from "react";
import CreationForm from "./CreationForm";
import { z } from "zod";

const schema = z.object({
  title: z
    .string()
    .min(5, { message: "Title needs to be at least 5 chars." })
    .max(69, { message: "Title cannot exceed 69 chars." }),
  details: z
    .string()
    .min(5, { message: "Details needs to be at least 5 chars." })
    .max(420, { message: "Details cannot exceed 420 chars." }),
});

export type CreationFormErrors = z.inferFlattenedErrors<typeof schema>;

const Page: FC = () => {
  const submit = async (
    _: CreationFormErrors,
    formData: FormData
  ): Promise<CreationFormErrors> => {
    "use server";

    const data = schema.safeParse({
      title: formData.get("title"),
      details: formData.get("details"),
    });

    if (data.success) {
      const ad = await prisma.ad.create({
        data: data.data,
      });

      redirect(`/editor/ad/${ad.pid}`);
    } else {
      return data.error.formErrors;
    }
  };

  return (
    <>
      <Heading>Create a new ad</Heading>

      <CreationForm submit={submit} />
    </>
  );
};

export default Page;
