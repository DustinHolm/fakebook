import { FC, memo } from "react";

type Props = {
  children: string;
};

const FormButton: FC<Props> = (props) => {
  return (
    <button
      type="submit"
      className="p-2 border-2 rounded border-gray-500 hover:border-teal-400"
    >
      {props.children}
    </button>
  );
};

export default memo(FormButton);
