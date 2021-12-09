import React from 'react';
import './App.css';
import {BrowserRouter} from "react-router-dom";
import { Provider } from 'react-redux';
import {store} from "./ducks/store";
import {Home} from "./pages/Home";
import {Settings} from "./containers/Settings";
import {Header} from "./containers/Header";
import Container from '@mui/material/Container';

function App() {
  return (
      <BrowserRouter>
        <Provider store={store}>
            <Header />
            <Container fixed>
                <Home />
            </Container>
            <Settings />
        </Provider>
      </BrowserRouter>
  );
}

export default App;
