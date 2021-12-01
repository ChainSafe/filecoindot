import { RootState } from "../store";
import {createSelector} from "@reduxjs/toolkit";

export const getSubstrateState = (state: RootState) => state.substrate.state;

export const getAccounts = (state: RootState) => state.substrate.accounts;

export const getSelectedAccountIndex = (state: RootState) => state.substrate.selectedAccountIndex;

export const getAccount = createSelector(
    getAccounts,
    getSelectedAccountIndex,
    (accounts, index) => index !== -1 ? accounts[index] : undefined,
);
