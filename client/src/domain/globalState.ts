import { produce } from "immer";
import { StateCreator, create } from "zustand";

type CurrentUserSlice = {
  id: string | null;
  setId: (newId: string) => void;
  unsetId: () => void;
};

const currentUserSlice: StateCreator<GlobalState, [], [], CurrentUserSlice> = (
  set
) => ({
  id: "MUFwcFVzZXI=", //placeholder for now
  setId: (id: string) => {
    set(
      produce((state: GlobalState): void => {
        state.currentUser.id = id;
      })
    );
  },
  unsetId: () => {
    set(
      produce((state: GlobalState) => {
        state.currentUser.id = null;
      })
    );
  },
});

type GlobalState = {
  currentUser: CurrentUserSlice;
};

export const useGlobalState = create<GlobalState>()((...all) => ({
  currentUser: { ...currentUserSlice(...all) },
}));
