import React, { useState, useEffect, createContext, useContext, useCallback } from "react"
import { web3Accounts, web3Enable } from "@polkadot/extension-dapp"
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types"
import { DAPP_NAME } from "../constants/substrate"

const LOCALSTORAGE_KEY = "fsb.selectedAccount"

type AccountContextProps = {
  children: React.ReactNode | React.ReactNode[]
}

export interface IAccountContext {
  selectedAddress?: string
  accountList?: InjectedAccountWithMeta[]
  selectAccount: (address: string) => void
  getAccountByAddress: (address: string) => InjectedAccountWithMeta | undefined
  isAccountLoading: boolean
  extensionNotFound: boolean
  isAccountListEmpty: boolean
}

const AccountContext = createContext<IAccountContext | undefined>(undefined)

const AccountContextProvider = ({ children }: AccountContextProps) => {
  const [selectedAddress, setSelected] = useState<string>("")
  const [accountList, setAccountList] = useState<InjectedAccountWithMeta[]>([])
  const [isAccountLoading, setIsAccountLoading] = useState(false)
  const [extensionNotFound, setExtensionNotFound] = useState(false)
  const [isAccountListEmpty, setIsAccountListEmpty] = useState(false)

  const selectAccount = useCallback((address: string) => {
    localStorage.setItem(LOCALSTORAGE_KEY, address)
    setSelected(address)
  }, [])

  const getaccountList = useCallback(async (): Promise<undefined> => {
    const extensions = await web3Enable(DAPP_NAME)

    if (extensions.length === 0) {
      setExtensionNotFound(true)
      setIsAccountLoading(false)
      return
    } else {
      setExtensionNotFound(false)
    }

    const accountList = await web3Accounts()

    if (accountList.length === 0) {
      setIsAccountListEmpty(true)
      setIsAccountLoading(false)
      return
    }

    setAccountList(accountList)

    if (accountList.length > 0) {
      const previousAccountAddress = localStorage.getItem(LOCALSTORAGE_KEY)

      if(!previousAccountAddress){
        selectAccount(accountList[0].address)
      } else {        selectAccount(previousAccountAddress)
      }

    }

    setIsAccountLoading(false)
    return
  }, [selectAccount])

  const getAccountByAddress = useCallback((address: string) => {
    return accountList.find(account => account.address === address)
  }, [accountList])

  useEffect(() => {
    if (!accountList.length) {
      getaccountList()
    }
  }, [accountList, getaccountList])

  return (
    <AccountContext.Provider
      value={{
        selectedAddress,
        accountList,
        selectAccount,
        isAccountLoading,
        extensionNotFound,
        isAccountListEmpty,
        getAccountByAddress
      }}
    >
      {children}
    </AccountContext.Provider>
  )
}

const useAccountList = () => {
  const context = useContext(AccountContext)
  if (context === undefined) {
    throw new Error("useAccountList must be used within a AccountContextProvider")
  }
  return context
}

export { AccountContextProvider, useAccountList }
