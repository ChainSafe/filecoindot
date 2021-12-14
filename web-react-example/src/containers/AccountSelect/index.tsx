import React from "react";
import {useDispatch, useSelector} from "react-redux";
import {getAccounts} from "../../ducks/substrate/selectors";
import {CardHeader, IconButton, Menu, MenuItem} from "@mui/material";
import { Identicon } from "@polkadot/react-identicon";
import {setSelectedAccountIndex} from "../../ducks/substrate/actions";
import {MenuProps} from "@mui/material/Menu/Menu";

interface Props extends Omit<MenuProps, 'open' | 'onClose' | 'anchorEl'> {
    anchorEl: null | HTMLElement;
    onClose: () => void;
}

export const AccountSelect: React.FC<Props> = ({ anchorEl, onClose, ...props}) => {
    const accounts = useSelector(getAccounts);
    const dispatch = useDispatch();

    const handleSelect = (address: string, index: number) => () => {
        dispatch(setSelectedAccountIndex(index));
        onClose();
    };

    return (
            <Menu
                anchorEl={anchorEl}
                open={Boolean(anchorEl)}
                onClose={onClose}
                {...props}
            >
                {accounts.map((account, index) => (
                    <MenuItem key={account.address} onClick={handleSelect(account.address, index)}>
                        <CardHeader avatar={
                            <IconButton sx={{p: 0}}>
                                <Identicon value={account.address} theme="substrate" size={40} />
                            </IconButton>
                        }
                                    title={account.meta.name} subheader={account.address} />
                    </MenuItem>
                ))}
            </Menu>
    );
};
