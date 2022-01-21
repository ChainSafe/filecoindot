import React from "react"
import "./App.css"
import { BrowserRouter, Route, Routes } from "react-router-dom"
import { MintNFT } from "./pages/MintNFT"
import { Header } from "./containers/Header"
import Container from "@mui/material/Container"
import { VerifyBlock } from "./pages/VerifyBlock"
import { UserSpace } from "./containers/UserSpace"
import { AccountContextProvider } from "./contexts/AccountsContext"
import { ApiContextProvider } from "./contexts/ApiContext"
import { NftContextProvider } from "./contexts/NftContext"

function App() {
  return (
    <BrowserRouter>
      <AccountContextProvider>
        <NftContextProvider>
          <ApiContextProvider>
            <Header />
            <Container
              fixed
              sx={{ paddingTop: "6rem" }}>
              <UserSpace>
                <Routes>
                  <Route
                    path="/"
                    element={<VerifyBlock />}
                  />
                  <Route
                    path="/nft"
                    element={<MintNFT />}
                  />
                </Routes>
              </UserSpace>
            </Container>
          </ApiContextProvider>
        </NftContextProvider>
      </AccountContextProvider>
    </BrowserRouter>
  )
}

export default App
