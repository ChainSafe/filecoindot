import React from "react"
import "./App.css"
import { BrowserRouter, Route, Routes } from "react-router-dom"
import { NFTList } from "./pages/NFTList"
// import { Settings } from './containers/Settings';
import { Header } from "./containers/Header"
import Container from "@mui/material/Container"
import { MintNFT } from "./pages/VerifyBlock"
import { UserSpace } from "./containers/UserSpace"
import { AccountContextProvider } from "./contexts/AccountsContext"
import { ApiContextProvider } from "./contexts/ApiContext"

function App() {
  return (
    <BrowserRouter>
      <AccountContextProvider>
        <ApiContextProvider>
          <Header />
          <Container
            fixed
            sx={{ paddingTop: "6rem" }}>
            <UserSpace>
              <Routes>
                <Route
                  path="/"
                  element={<MintNFT />}
                />
                <Route
                  path="/list"
                  element={<NFTList />}
                />
              </Routes>
            </UserSpace>
          </Container>
          {/* <Settings /> */}
        </ApiContextProvider>
      </AccountContextProvider>
    </BrowserRouter>
  )
}

export default App
