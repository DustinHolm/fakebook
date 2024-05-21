import { AdInteraction } from "@prisma/client";
import { FC, memo } from "react";

type Props = {
  interactions: AdInteraction[];
};

const InteractionTable: FC<Props> = (props) => {
  return (
    <table className="table-fixed w-full border border-teal-400">
      <thead>
        <tr>
          <th className="w-1/2 border border-teal-400">Time</th>

          <th className="w-1/2 border border-teal-400">Type</th>
        </tr>
      </thead>

      <tbody>
        {props.interactions.map((interaction) => (
          <tr key={interaction.pid}>
            <td className="border border-teal-400">
              {interaction.time.toISOString()}
            </td>

            <td className="border border-teal-400">
              {interaction.interactionType}
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default memo(InteractionTable);
