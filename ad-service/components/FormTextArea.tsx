import { DetailedHTMLProps, FC, TextareaHTMLAttributes, memo } from "react";

type Props = {
  id: string;
  label: string;
  rows: number;
  errors?: string[];
} & Omit<
  DetailedHTMLProps<
    TextareaHTMLAttributes<HTMLTextAreaElement>,
    HTMLTextAreaElement
  >,
  "name" | "rows" | "className"
>;

const FormTextArea: FC<Props> = (props) => {
  return (
    <div>
      <div className="flex flex-row space-x-4 items-start">
        <label htmlFor={props.id} className="flex-grow-0">
          {props.label}
        </label>

        <textarea
          {...props}
          name={props.id}
          rows={props.rows}
          className="resize-none flex-grow border border-transparent border-b-teal-400 p-1 bg-gray-500/20"
        />
      </div>

      {props.errors?.length &&
        props.errors.length > 0 &&
        props.errors.map((error) => (
          <p key={error} className="text-red-500 text-sm">
            {error}
          </p>
        ))}
    </div>
  );
};

export default memo(FormTextArea);
