import FormButton from "$/components/FormButton";
import FormInput from "$/components/FormInput";
import FormTextArea from "$/components/FormTextArea";
import Heading from "$/components/Heading";
import prisma from "$/lib/prisma";
import { redirect } from "next/navigation";
import { FC } from "react";

const Page: FC = async () => {
  const submit = async (formData: FormData) => {
    "use server";
    const title = formData.get("title")?.toString();
    const description = formData.get("description")?.toString();

    if (!title || !description) {
      throw Error("What?! Somebody hacked the frontend and submitted junk!");
    }

    const ad = await prisma.ad.create({
      data: {
        title,
        details: description,
      },
    });

    redirect(`/ad/${ad.pid}`);
  };

  return (
    <>
      <Heading>Create a new ad</Heading>

      <form action={submit} className="flex flex-col items-end space-y-4">
        <div className="flex flex-col w-full space-y-8 border-gray-500 border-2 rounded p-8">
          <FormInput id="title" label="Title" required />

          <FormTextArea
            id="description"
            label="Description"
            rows={3}
            required
          />
        </div>

        <FormButton>Create</FormButton>
      </form>
    </>
  );
};

export default Page;
