import React from 'react';
import './App.css';
import {BrowserRouter} from "react-router-dom";
import { Provider } from 'react-redux';
import {store} from "./ducks/store";
import {Home} from "./pages/Home";

function App() {
  return (
      <BrowserRouter>
        <Provider store={store}>
            <Home />
        </Provider>
      </BrowserRouter>
  );
}

export default App;
