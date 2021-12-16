import React from 'react';
import './App.css';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import { Home } from './pages/Home';
import { Settings } from './containers/Settings';
import { Header } from './containers/Header';
import Container from '@mui/material/Container';
import { MintNFT } from './pages/MintNFT';
import { UserSpace } from './containers/UserSpace';
import { AccountContextProvider } from './contexts/AccountsContext';

function App() {
  return (
    <BrowserRouter>
      {/* <Provider store={store}> */}
      <AccountContextProvider>
        <Header />
        <Container fixed sx={{ paddingTop: '6rem' }}>
          <UserSpace>
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/add" element={<MintNFT />} />
            </Routes>
          </UserSpace>
        </Container>
        <Settings />
      </AccountContextProvider>
      {/* </Provider> */}
    </BrowserRouter>
  );
}

export default App;
