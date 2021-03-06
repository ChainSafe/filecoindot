import React from "react"
import { ApiPromise, WsProvider } from "@polkadot/api"
import { ApiOptions } from "@polkadot/api/types"
import { TypeRegistry } from "@polkadot/types"
import { useState, useEffect, createContext, useContext } from "react"
import { useDidUpdateEffect } from "../hooks/useDidUpdateEffect"
import { rpc } from "@chainsafe/filecoindot-types"

type ApiContextProps = {
  children: React.ReactNode | React.ReactNode[]
  types?: ApiOptions["types"]
}

const registry = new TypeRegistry()

export interface IApiContext {
  api: ApiPromise // From @polkadot/api\
  isApiReady: boolean
}

const ApiContext = createContext<IApiContext | undefined>(undefined)


const ApiContextProvider = ({ children, types }: ApiContextProps) => {
  const WS_PROVIDER = process.env.REACT_APP_WS_PROVIDER
  const provider = new WsProvider(WS_PROVIDER)
  const [apiPromise, setApiPromise] = useState<ApiPromise>(
    new ApiPromise({ provider, types, rpc })
  )
  const [isReady, setIsReady] = useState(false)

  useDidUpdateEffect(() => {
    // We want to fetch all the information again each time we reconnect. We
    // might be connecting to a different node, or the node might have changed
    // settings.
    setApiPromise(new ApiPromise({ provider, types, rpc }))

    setIsReady(false)
  })

  useEffect(() => {
    // We want to fetch all the information again each time we reconnect. We
    // might be connecting to a different node, or the node might have changed
    // settings.
    apiPromise.isReady
      .then(() => {
        if (types) {
          registry.register(types)
        }

        setIsReady(true)
      })
      .catch(e => console.error(e))
  }, [apiPromise.isReady, types])


  if (!WS_PROVIDER) {
    console.error("REACT_APP_WS_PROVIDER not set")
    return null
  }

  return (
    <ApiContext.Provider
      value={{
        api: apiPromise,
        isApiReady: isReady
      }}
    >
      {children}
    </ApiContext.Provider>
  )
}

const useApi = () => {
  const context = useContext(ApiContext)
  if (context === undefined) {
    throw new Error("useApi must be used within a ApiContextProvider")
  }
  return context
}

export { ApiContextProvider, useApi }
