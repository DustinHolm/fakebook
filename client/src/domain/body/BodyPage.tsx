import { memo } from "react";
import { Outlet } from "react-router";
import Body from "../../components/Body";

function BodyPage() {
  return (
    <Body>
      <Outlet />
    </Body>
  );
}

export default memo(BodyPage);
