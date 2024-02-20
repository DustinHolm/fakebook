import { FormEvent } from "react";

export type SubmitData = { [key: string]: string | undefined };

export function submit(
  event: FormEvent<HTMLFormElement>,
  callback: (data: SubmitData) => void
) {
  event.preventDefault();
  const formData = new FormData(event.currentTarget);

  const dataObject: SubmitData = {};

  for (const [key, value] of formData) {
    dataObject[key] = value as string | undefined;
  }

  callback(dataObject);
}
