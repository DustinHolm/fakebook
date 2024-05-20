"use client";
import FormButton from "$/components/FormButton";
import FormInput from "$/components/FormInput";
import FormTextArea from "$/components/FormTextArea";
import { FC, memo } from "react";
import { useFormState } from "react-dom";
import { CreationFormErrors } from "./page";

type Props = {
  submit: (
    prev: CreationFormErrors,
    data: FormData
  ) => Promise<CreationFormErrors>;
};

const CreationForm: FC<Props> = (props) => {
  const [formState, formSubmit] = useFormState<CreationFormErrors, FormData>(
    props.submit,
    {
      formErrors: [],
      fieldErrors: {},
    }
  );

  return (
    <form action={formSubmit} className="flex flex-col items-end space-y-4">
      <div className="flex flex-col w-full space-y-8 border-gray-500 border-2 rounded p-8">
        <FormInput
          id="title"
          label="Title"
          required
          errors={formState.fieldErrors.title}
        />

        <FormTextArea
          id="details"
          label="Details"
          rows={3}
          required
          errors={formState.fieldErrors.details}
        />
      </div>
      <FormButton>Create</FormButton>
    </form>
  );
};

export default memo(CreationForm);
