import { FC, memo } from "react";

type Props = {
  children: string;
};

const Heading: FC<Props> = (props) => {
  return (
    <>
      <h1 className="text-4xl text-center">{props.children}</h1>
      <hr className="mt-4 mb-8" />
    </>
  );
};

export default memo(Heading);
