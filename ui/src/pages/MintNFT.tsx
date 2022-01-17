/* eslint-disable max-len */
import { Box, CircularProgress, TextField, Typography } from "@mui/material"
import LoadingButton from "@mui/lab/LoadingButton"
import React, { ChangeEvent, useCallback, useEffect, useState } from "react"
import { Center } from "../components/layout/Center"
import { useApi } from "../contexts/ApiContext"
import CheckCircleIcon from "@mui/icons-material/CheckCircle"
// import AutoAwesomeIcon from "@mui/icons-material/AutoAwesome"
import CancelIcon from "@mui/icons-material/Cancel"
// import { proofJSON } from "../proof"
// import { web3FromSource } from "@polkadot/extension-dapp"
// import { useAccountList } from "../contexts/AccountsContext"

export const MintNFT = () => {
  const [cid, setCid] = useState("")
  // const [proof, setProof] = useState(proofJSON.proof)
  const { api, isApiReady } = useApi()
  const [isLoading, setIsLoading] = useState(false)
  const [isValid, setIsValid] = useState<boolean | null>(null)
  const [error, setError] = useState("")
  // const { selectedAddress, getAccountByAddress } = useAccountList()
  const [mintedBlock, setMintedBlock] = useState("")
  const [isMinting, setIsMinting] = useState(false)
  const [cidMap, setCidMap] = useState<string[]>([])

  const resetState = useCallback(() => {
    setIsValid(null)
    setError("")
    setMintedBlock("")
    setIsMinting(false)
  }, [])

  const onChangeCid = useCallback((cid: ChangeEvent<HTMLInputElement>) => {
    setCid(cid.currentTarget.value)
    resetState()
  }, [resetState])

  const refreshCidMap = useCallback(() =>
    api.query.filecoindot.verifiedBlocks.entries()
      .then((res) => {
        const arr = res.map(([res1]) => (res1.toHuman() as string[])[0])
        setCidMap(arr)
        // console.log("res2", res2)
        return arr
      })
      .catch((e: any) => {
        setError(e.message)
        console.error(e)
      })
  , [api])

  useEffect(() => {
    if(isApiReady && !cidMap.length) {
      refreshCidMap()
    }
  }, [cidMap, isApiReady, refreshCidMap])
  // const onChangeProof = useCallback((proof: ChangeEvent<HTMLInputElement>) => {
  //   setProof(proof.currentTarget.value)
  //   resetState()
  // }, [resetState])

  const onVerify = useCallback(() => {
    if (!isApiReady) {
      console.error("Api is not connected")
      setError("Api not connected to your node")
      return
    }

    setIsLoading(true)

    const isThereAlready = !!cidMap.find(c => cid === c)

    if(isThereAlready) {
      setIsValid(true)
      setIsLoading(false)
      return
    }

    refreshCidMap()
      .then((arr) => {
        if(!arr) return

        setIsValid(!!arr.find(c => cid === c))
        setIsLoading(false)
      })
  }, [cid, cidMap, isApiReady, refreshCidMap])

  // const onMint = useCallback(async () => {
  //   if (!selectedAddress) return

  //   const signerAccount = getAccountByAddress(selectedAddress)

  //   if (!signerAccount) return

  //   setError("")
  //   setIsMinting(true)

  // const injector = await web3FromSource(signerAccount.meta.source)

  // api.tx.filecoindotNFT
  //   .mint(cid, [proof])
  //   .signAndSend(signerAccount.address, { signer: injector.signer }, ({ status }) => {
  //     console.log("status", status)

  //     if(status.isInBlock) {
  //       setMintedBlock(status.asInBlock.toString())
  //     }
  //   })
  //   .catch((e: Error) => {
  //     setError(e.message)
  //     console.error(e)
  //     setIsMinting(false)
  //   })

  // }, [getAccountByAddress, selectedAddress])

  return (
    <Center>
      <h1>Verify a Filecoin block</h1>
      <h4>indexed cid: {cidMap.length || <CircularProgress size={14}/>}</h4>
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          "& .MuiTextField-root": { marginBottom: "2rem" }
        }}
      >
        <TextField
          autoFocus
          fullWidth
          required
          id="cid"
          label="cid"
          placeholder="cid"
          onChange={onChangeCid}
          value={cid}
        />
        {/* <TextField
          fullWidth
          required
          id="proof"
          label="Proof"
          placeholder=""
          onChange={onChangeProof}
          // value={proof}
          value={proofJSON.proof}
        /> */}
        {error && (
          <Typography
            variant="h6"
            noWrap
            component="div"
            sx={{
              color: "firebrick",
              display: "flex",
              alignContent: "center",
              justifyContent: "center",
              alignItems: "center"
            }}
          >
            {error}
          </Typography>
        )}
        {!error && isValid === null
          ? (
            <LoadingButton
              variant="contained"
              onClick={onVerify}
              disabled={!cid || !isApiReady || isLoading}
              loading={!!isLoading}
              loadingPosition="center"
            >
              {isLoading
                ? "Verifying"
                : "Verify"
              }
            </LoadingButton>
          )
          : <Typography
            variant="h6"
            noWrap
            component="div"
            sx={{
              color: isValid ? "lightseagreen" : "firebrick",
              display: "flex",
              alignContent: "center",
              justifyContent: "center",
              alignItems: "center",
              flexDirection: "column",
              "&:first-of-type": { marginBottom: "1rem" }
            }}
          >
            {!mintedBlock && !isMinting && (
              isValid
                ? (!error &&
                      <>
                        <CheckCircleIcon fontSize="large"/>
                        This block&apos;s cid was found!
                      </>
                )
                : (
                  <>
                    <CancelIcon fontSize="large" />
                    This cid wasn&apos;t found in our data set :(
                  </>
                ))
            }
            {/* {!mintedBlock && isMinting && (
              <Box sx={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                "&: last-child": {
                  marginTop: "1rem",
                  color: "black"
                }
              }}
              >
                <CircularProgress />
                Minting your NFT...
              </Box>
            )}
            {mintedBlock &&
              <>
                <AutoAwesomeIcon fontSize="large"/>
                  NFT minted at block: {mintedBlock}
                <Button
                  variant="contained"
                  onClick={resetState}
                  sx={{ marginTop: "1rem" }}
                >
                  Verify and Mint another NFT
                </Button>
              </>
            } */}
          </Typography>
        }
      </Box>
    </Center>
  )
}