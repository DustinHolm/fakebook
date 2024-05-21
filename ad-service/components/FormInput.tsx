import { DetailedHTMLProps, FC, InputHTMLAttributes, memo } from "react";

type Props = {
  id: string;
  label: string;
  errors?: string[];
} & Omit<
  DetailedHTMLProps<InputHTMLAttributes<HTMLInputElement>, HTMLInputElement>,
  "type" | "name" | "className"
>;

const FormInput: FC<Props> = (props) => {
  return (
    <div>
      <div className="flex flex-row space-x-4 items-center">
        <label htmlFor={props.id} className="flex-grow-0">
          {props.label}
        </label>

        <input
          {...props}
          type="text"
          name={props.id}
          className="flex-grow border border-transparent border-b-teal-400 p-1 bg-gray-500/20"
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

export default memo(FormInput);
