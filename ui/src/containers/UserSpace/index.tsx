import React from 'react';
import { Center } from '../../components/layout/Center';
import { Button } from '@mui/material';
import KeyboardArrowDownIcon from '@mui/icons-material/KeyboardArrowDown';
import { AccountSelect } from '../AccountSelect';
import { useAccountList } from '../../contexts/AccountsContext';

export const UserSpace: React.FC = ({ children }) => {
  const {extensionNotFound, isAccountListEmpty, isAccountLoading, selected} = useAccountList()
  const [anchorElUser, setAnchorElUser] = React.useState<null | HTMLElement>(null);
  const handleOpenUserMenu = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorElUser(event.currentTarget);
  };
  const handleCloseUserMenu = () => {
    setAnchorElUser(null);
  };

  if (selected) return <>{children}</>;

  if(isAccountLoading)
  return (
    <Center>
      <h1>Loading...</h1>
    </Center>
  );

  if(extensionNotFound)
  return (
    <Center>
      <h1>Please install the extension</h1>
    </Center>
  );

  if(isAccountListEmpty)
  return (
    <Center>
      <h1>Please create at least an account</h1>
    </Center>
  );

  // if (state === PluginState.INITIALIZATION) return null;

  // if (state === PluginState.UNAUTHORIZED)
  //   return (
  //     <Center>
  //       <h1>Please Authorise page</h1>
  //     </Center>
  //   );

  // if (state === PluginState.NONE)
  //   return (
  //     <Center>
  //       <h1>There is no plugin :sad:</h1>
  //     </Center>
  //   );

  // if (state === PluginState.INJECTED)
  //   return (
  //     <Center>
  //       <h1>Please Allow Access</h1>
  //     </Center>
  //   );

  // if (!accounts.length)
  //   return (
  //     <Center>
  //       <h1>Please Add Account</h1>
  //     </Center>
  //   );

  return (
    <Center>
      <Button variant="outlined" endIcon={<KeyboardArrowDownIcon />} onClick={handleOpenUserMenu}>
        Select an account
      </Button>
      <AccountSelect anchorEl={anchorElUser} onClose={handleCloseUserMenu} />
    </Center>
  );
};
