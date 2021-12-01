import React from "react";
import {useSelector} from "react-redux";
import {getAccount, getAccounts, getSubstrateState} from "../ducks/substrate/selectors";
import {PluginState} from "../ducks/substrate/slice";

export const Home: React.FC = () => {
    const state = useSelector(getSubstrateState);
    const accounts = useSelector(getAccounts);
    const selectedAccount = useSelector(getAccount);

    if (state === PluginState.INITIALIZATION) return null;

    if (state === PluginState.UNAUTHORIZED)
        return (
            <div>
                <h1>Please Authorise page</h1>
            </div>
        );

    if (state === PluginState.NONE)
        return (
            <div>
                <h1>There is no plugin :sad:</h1>
            </div>
        );

    if (state === PluginState.INJECTED)
        return (
            <div>
                <h1>Please Allow Access</h1>
            </div>
        );

    if (!accounts.length)
        return (
            <div>
                <h1>Please Add Account</h1>
            </div>
        );

    if (selectedAccount)
        return (
            <div>
                <h1>Selected Account</h1>
                <h3>{selectedAccount.address}</h3>
                <h3>{selectedAccount.meta.name}</h3>
            </div>
        );

    return (
        <div>
            <h1>Home</h1>
            {accounts.map((account) => <p key={account.address}>{account.address} =&gt; {account.meta.name}</p>)}
        </div>
    );
}
