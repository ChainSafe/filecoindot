import React from 'react';
import './App.css';
import {BrowserRouter, Route, Routes} from "react-router-dom";
import { Provider } from 'react-redux';
import {store} from "./ducks/store";
import {Home} from "./pages/Home";
import {Settings} from "./containers/Settings";
import {Header} from "./containers/Header";
import Container from '@mui/material/Container';
import {MintNFT} from "./pages/MintNFT";
import {UserSpace} from "./containers/UserSpace";

function App() {
  return (
      <BrowserRouter>
        <Provider store={store}>
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
        </Provider>
      </BrowserRouter>
  );
}

export default App;
