import Link from "next/link";
import { FC, memo } from "react";

type Props = {
  addresses: {
    name: string;
    href: string;
  }[];
};

const NavBar: FC<Props> = (props) => {
  return (
    <div className="flex justify-center space-x-6 mt-2 mb-2">
      {props.addresses.map(({ name, href }) => (
        <Link
          key={name}
          href={href}
          className="hover:underline decoration-teal-400"
        >
          {name}
        </Link>
      ))}
    </div>
  );
};

export default memo(NavBar);
