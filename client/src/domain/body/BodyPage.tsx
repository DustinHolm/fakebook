import { memo } from "react";
import { Outlet } from "react-router";
import { Body } from "$components/Body";

function _BodyPage() {
  return (
    <Body>
      <Outlet />
    </Body>
  );
}

export const BodyPage = memo(_BodyPage);
