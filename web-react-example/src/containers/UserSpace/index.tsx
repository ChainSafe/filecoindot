import React from 'react';
import { useSelector } from 'react-redux';
import { getAccount, getAccounts, getSubstrateState } from '../../ducks/substrate/selectors';
import { PluginState } from '../../ducks/substrate/slice';
import { Center } from '../../components/layout/Center';
import { Button } from '@mui/material';
import KeyboardArrowDownIcon from '@mui/icons-material/KeyboardArrowDown';
import { AccountSelect } from '../AccountSelect';

export const UserSpace: React.FC = ({ children }) => {
  const state = useSelector(getSubstrateState);
  const accounts = useSelector(getAccounts);
  const selectedAccount = useSelector(getAccount);

  const [anchorElUser, setAnchorElUser] = React.useState<null | HTMLElement>(null);
  const handleOpenUserMenu = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorElUser(event.currentTarget);
  };
  const handleCloseUserMenu = () => {
    setAnchorElUser(null);
  };

  if (selectedAccount) return <>{children}</>;

  if (state === PluginState.INITIALIZATION) return null;

  if (state === PluginState.UNAUTHORIZED)
    return (
      <Center>
        <h1>Please Authorise page</h1>
      </Center>
    );

  if (state === PluginState.NONE)
    return (
      <Center>
        <h1>There is no plugin :sad:</h1>
      </Center>
    );

  if (state === PluginState.INJECTED)
    return (
      <Center>
        <h1>Please Allow Access</h1>
      </Center>
    );

  if (!accounts.length)
    return (
      <Center>
        <h1>Please Add Account</h1>
      </Center>
    );

  return (
    <Center>
      <Button variant="outlined" endIcon={<KeyboardArrowDownIcon />} onClick={handleOpenUserMenu}>
        Please select account
      </Button>
      <AccountSelect anchorEl={anchorElUser} onClose={handleCloseUserMenu} />
    </Center>
  );
};
