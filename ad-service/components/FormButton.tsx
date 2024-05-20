"use client";
import { FC, memo } from "react";
import { useFormStatus } from "react-dom";
import Spinner from "./Spinner";

type Props = {
  children: string;
};

const FormButton: FC<Props> = (props) => {
  const { pending } = useFormStatus();

  return (
    <button
      type="submit"
      className="p-2 border-2 rounded border-gray-500 hover:border-teal-400 flex items-center"
      disabled={pending}
    >
      {pending && (
        <span className="mr-2">
          <Spinner />
        </span>
      )}
      {props.children}
    </button>
  );
};

export default memo(FormButton);
