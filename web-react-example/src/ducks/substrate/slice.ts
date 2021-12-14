import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';

export enum PluginState {
  INITIALIZATION,
  INJECTED,
  AUTHORIZED,
  UNAUTHORIZED,
  NONE,
}

interface SubstrateState {
  state: PluginState;
  accounts: InjectedAccountWithMeta[];
  selectedAccountIndex: number;
}

const initialState: SubstrateState = {
  state: PluginState.INITIALIZATION,
  accounts: [],
  selectedAccountIndex: -1,
};

export const substrateSlice = createSlice({
  name: 'substrate',
  initialState,
  reducers: {
    setSubstrateState: (state, action: PayloadAction<PluginState>) => {
      state.state = action.payload;
    },
    setSelectedAccountIndex: (state, action: PayloadAction<number>) => {
      state.selectedAccountIndex = action.payload;
    },
    setAccounts: (state, action: PayloadAction<InjectedAccountWithMeta[]>) => {
      state.accounts = action.payload;
    },
  },
});
