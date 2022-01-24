import React, { useState, createContext, useContext, useCallback } from "react"

type AccountContextProps = {
  children: React.ReactNode | React.ReactNode[]
}

interface Nft {
  filePreview: string
  name: string
  cid: string
}
export interface INftContext {
  nftList: Nft[]
  addNft: (nft: Nft) => void
}

const NftContext = createContext<INftContext | undefined>(undefined)

const NftContextProvider = ({ children }: AccountContextProps) => {
  const [nftList, setNftList] = useState<Nft[]>([])

  const addNft = useCallback((nft: Nft) => {
    setNftList((prevList) => [...prevList, nft])
  }, [])

  return (
    <NftContext.Provider
      value={{
        nftList,
        addNft
      }}
    >
      {children}
    </NftContext.Provider>
  )
}

const useNftList = () => {
  const context = useContext(NftContext)
  if (context === undefined) {
    throw new Error("useAccountList must be used within a NftContextProvider")
  }
  return context
}

export { NftContextProvider, useNftList }
