import { Box, Button, TextField } from '@mui/material';
import React from 'react';
import { Center } from '../components/layout/Center';

export const MintNFT: React.FC = () => {
  return (
    <Center>
      <h1>Mint new token</h1>
      <Box
        sx={{
          display: 'flex',
          flexDirection: 'column',
          '& .MuiTextField-root': { marginBottom: '2rem', width: '30rem' },
        }}
      >
        <TextField autoFocus fullWidth required id="name" label="Name" placeholder="Awesome NFT!" />
        <TextField
          fullWidth
          required
          id="cid"
          label="CID"
          placeholder="zb2rhe5P4gXftAwvA4eXQ5HJwsER2owDyS9sKaQRRVQPn93bA"
        />
        <Button variant="contained">Verify</Button>
      </Box>
    </Center>
  )
};
