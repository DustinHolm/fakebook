import { produce } from "immer";
import { StateCreator, create } from "zustand";

type CurrentUserSlice = {
  id: number | null;
  setId: (newId: number) => void;
  unsetId: () => void;
};

const currentUserSlice: StateCreator<GlobalState, [], [], CurrentUserSlice> = (
  set
) => ({
  id: 1, //placeholder for now
  setId: (id: number) => {
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
