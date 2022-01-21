import { Keyring } from "@polkadot/api"
import { web3FromSource } from "@polkadot/extension-dapp"
import { useCallback, useState } from "react"
import { useAccountList } from "../contexts/AccountsContext"
import { useApi } from "../contexts/ApiContext"
import useBuckets from "./useBuckets"

const COLLECTION_ID = "d43593c7a56da27d-FILDOT"
const COLLECTION = {
  "max": 0,
  "issuer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  "symbol": "FILDOT",
  "id": COLLECTION_ID,
  "metadata": "ipfs://ipfs/QmQCVGmrSTixoxBe8qg4McW3KxipUZq9jugucHyfmyPGrk"
}

const useNft = () => {
  const [isCollectionCreated, setIsCollectionCreated] = useState(false)
  const { isApiReady, api } = useApi()
  const { selectedAddress, getAccountByAddress } = useAccountList()
  const { nftMetaBucketId, uploadFile } = useBuckets()

  const createCollection = async () => {
    if (isCollectionCreated || !isApiReady) return

    console.log("Creating collection with Alice")

    const payload = `RMRK::MINT::2.0.0::${encodeURIComponent(JSON.stringify(COLLECTION))}`
    const keyring = new Keyring({ type: "sr25519" })
    const signer = keyring.createFromUri("//Alice")
    return new Promise<void>((resolve, reject) => {
      api.tx.system.remark(payload)
        .signAndSend(signer, ({ status }) => {
          if(status.isInBlock) {
            console.log("collection created at block", status.asInBlock.toString())
            setIsCollectionCreated(true)
            resolve()
          }
        })
        .catch((e: Error) => {
          console.error(e)
          reject(e)
        })
    })
  }

  const uploadNftMetadata = useCallback((imageCid: string, name: string) => {
    const nftMetadata = {
      "external_url": "https://github.com/ChainSafe/filecoindot",
      "image": `ipfs://ipfs/${imageCid}`,
      "description": "This Nft was created using the Filecoindot demo",
      "name": name,
      "properties": {}
    }

    const jsonNftMetadata = JSON.stringify(nftMetadata)
    const jsonNftMetadataFile = new File([jsonNftMetadata], `${name}_Metadata.json`, { type: "application/json" })

    return uploadFile(jsonNftMetadataFile, nftMetaBucketId)
  }, [nftMetaBucketId, uploadFile])


  const mintNft = useCallback(async (imageCid: string, name: string) => {

    if (!selectedAddress) return

    const nftMetadataCid = await uploadNftMetadata(imageCid, name)
    const signerAccount = getAccountByAddress(selectedAddress)

    if (!signerAccount) {
      throw new Error("No account selected")
    }

    const injector = await web3FromSource(signerAccount.meta.source)

    const nft = {
      "collection": COLLECTION_ID,
      "transferable": 0,
      "sn": "00000001",
      "metadata": `ipfs://ipfs/${nftMetadataCid}`
    }

    const payload = `RMRK::MINT::2.0.0::${encodeURIComponent(JSON.stringify(nft))}`
    return new Promise<string>((resolve, reject) => {
      api.tx.system.remark(payload)
        .signAndSend(signerAccount.address, { signer: injector.signer }, ({ status }) => {
          if(status.isInBlock) {
            console.log("Nft minted at block", status.asInBlock.toString())
            resolve(status.asInBlock.toString())
          }
        })
        .catch((e: Error) => {
          console.error(e)
          reject(e)
        })
    })
  }, [api, getAccountByAddress, uploadNftMetadata, selectedAddress])

  return { createCollection, isCollectionCreated, mintNft }
}

export default useNft

