import React, { useState, useEffect, createContext, useContext, useCallback } from "react"
import { web3Accounts, web3Enable } from "@polkadot/extension-dapp"
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types"
import { DAPP_NAME } from "../constants/substrate"

const LOCALSTORAGE_KEY = "fsb.selectedAccount"

type AccountContextProps = {
  children: React.ReactNode | React.ReactNode[]
}

export interface IAccountContext {
  selected?: InjectedAccountWithMeta
  accountList?: InjectedAccountWithMeta[]
  selectAccount: (address: string) => void
  isAccountLoading: boolean
  extensionNotFound: boolean
  isAccountListEmpty: boolean
}

const AccountContext = createContext<IAccountContext | undefined>(undefined)

const AccountContextProvider = ({ children }: AccountContextProps) => {
  const [selected, setSelected] = useState<InjectedAccountWithMeta | undefined>()
  const [accountList, setAccountList] = useState<InjectedAccountWithMeta[]>([])
  const [isAccountLoading, setIsAccountLoading] = useState(false)
  const [extensionNotFound, setExtensionNotFound] = useState(false)
  const [isAccountListEmpty, setIsAccountListEmpty] = useState(false)

  const selectAccount = useCallback((account: string | InjectedAccountWithMeta) => {
    if(typeof account === "string"){
      localStorage.setItem(LOCALSTORAGE_KEY, account)
      setSelected(accountList.find(a => a.address === account))
    } else {
      localStorage.setItem(LOCALSTORAGE_KEY, account.address)
      setSelected(account)
    }
  }, [accountList])

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

    // if addresses need to be encoded
    // accountList.forEach((account) => {
    //     account.address = encodeAddress(account.address) || account.address;
    // });

    setAccountList(accountList)

    if (accountList.length > 0) {
      const previousAccountAddress = localStorage.getItem(LOCALSTORAGE_KEY)

      console.log("previsou", previousAccountAddress)
      if(!previousAccountAddress){
        console.log("select first")
        selectAccount(accountList[0])
      } else {
        console.log("selecprvi", previousAccountAddress)
        selectAccount(previousAccountAddress)
      }

    }

    setIsAccountLoading(false)
    return
  }, [selectAccount])

  useEffect(() => {
    if (!accountList.length) {
      getaccountList()
    }
  }, [accountList, getaccountList])

  return (
    <AccountContext.Provider
      value={{
        selected,
        accountList,
        selectAccount,
        isAccountLoading,
        extensionNotFound,
        isAccountListEmpty
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
