import React, { useState, useEffect, createContext, useContext } from "react"
import { web3Accounts, web3Enable } from "@polkadot/extension-dapp"
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types"
import { DAPP_NAME } from "../constants/substrate"

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

  useEffect(() => {
    if (!accountList.length) {
      getaccountList()
    }
  }, [accountList.length])

  const getaccountList = async (): Promise<undefined> => {
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
      setSelected(accountList[0])
    }

    setIsAccountLoading(false)
    return
  }

  const selectAccount = (address: string) => {
    setSelected(accountList.find(account => account.address === address))
  }

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
