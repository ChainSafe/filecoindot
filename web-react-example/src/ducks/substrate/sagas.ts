import {all, call, CallEffect, put, PutEffect, takeEvery} from "redux-saga/effects";
import {isWeb3Injected, web3Accounts, web3Enable} from "@polkadot/extension-dapp";
import {postInit} from "../store";
import {DAPP_NAME} from "../../constants/substrate";
import {InjectedAccountWithMeta, InjectedExtension} from "@polkadot/extension-inject/types";
import {setAccounts, setSubstrateState} from "./actions";
import {PluginState} from "./slice";

function* initialization(): Generator<CallEffect | PutEffect, void, InjectedExtension[] & InjectedAccountWithMeta[]> {
    try {
        if (!isWeb3Injected) {
            yield put(setSubstrateState(PluginState.NONE));
            return;
        }
        yield put(setSubstrateState(PluginState.INJECTED));

        const extensions = yield call(web3Enable, DAPP_NAME);
        if (!extensions.length) throw new Error('Not authorised page');

        yield put(setSubstrateState(PluginState.AUTHORIZED));

        const accounts = yield call(web3Accounts);
        yield put(setAccounts(accounts));

        // on every account change (delete, add) will send new account list, we should handle it!
        // web3AccountsSubscribe(console.log);
    } catch (e) {
        yield put(setSubstrateState(PluginState.UNAUTHORIZED));
        console.error(e);
    }
}

export function* authSagaWatcher(): Generator {
    yield all([
        takeEvery(postInit, initialization),
    ]);
}
