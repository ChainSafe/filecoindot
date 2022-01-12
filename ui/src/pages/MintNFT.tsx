import { Box, Button, CircularProgress, TextField, Typography } from "@mui/material"
import React, { ChangeEvent, useCallback, useState } from "react"
import { Center } from "../components/layout/Center"
import { useApi } from "../contexts/ApiContext"
import CheckCircleIcon from "@mui/icons-material/CheckCircle"
import AutoAwesomeIcon from "@mui/icons-material/AutoAwesome"
import CancelIcon from "@mui/icons-material/Cancel"
import { proofJSON } from "../proof"
import { web3FromSource } from "@polkadot/extension-dapp"
import { useAccountList } from "../contexts/AccountsContext"

export const MintNFT = () => {
  const [cid, setCid] = useState(proofJSON.cid)
  const [proof, setProof] = useState(proofJSON.proof)
  const { api, isApiReady } = useApi()
  const [isLoading, setIsLoading] = useState(false)
  const [isValid, setIsValid] = useState<boolean | null>(null)
  const [error, setError] = useState("")
  const { selected } = useAccountList()
  const [mintedBlock, setMintedBlock] = useState("")
  const [isMinting, setIsMinting] = useState(false)

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

  const onChangeProof = useCallback((proof: ChangeEvent<HTMLInputElement>) => {
    setProof(proof.currentTarget.value)
    resetState()
  }, [resetState])

  const onVerify = useCallback(() => {
    if (!isApiReady) {
      console.error("Api is not connected")
      setError("Api not connected to your node")
      return
    }

    setIsLoading(true);

    (api.rpc as any).filecoindot.verifyState(proof, cid)
      .then((res: any) => {
        console.log(Boolean(res))
        setIsValid(!!res.toHuman())
      })
      .catch((e: any) => {
        setError(e.message)
        console.error(e)
      })
      .finally(() => setIsLoading(false))
  }, [api, cid, isApiReady, proof])

  const onMint = useCallback(async () => {
    if (!selected) return

    setError("")
    setIsMinting(true)

    const injector = await web3FromSource(selected.meta.source)

    api.tx.filecoindotNFT
      .mint(cid, [proof])
      .signAndSend(selected.address, { signer: injector.signer }, ({ status }) => {
        console.log("status", status)

        if(status.isInBlock) {
          setMintedBlock(status.asInBlock.toString())
        }
      })
      .catch((e: Error) => {
        setError(e.message)
        console.error(e)
        setIsMinting(false)
      })

  }, [api, cid, proof, selected])

  return (
    <Center>
      <h1>Verify your cid to before minting an NFT</h1>
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
          // value={cid}
          value={proofJSON.cid}
        />
        <TextField
          fullWidth
          required
          id="proof"
          label="Proof"
          placeholder=""
          onChange={onChangeProof}
          // value={proof}
          value={proofJSON.proof}
        />
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
            <Button
              variant="contained"
              onClick={onVerify}
              disabled={!cid || !proof || !isApiReady || isLoading}
            >
              {isLoading
                ? "Verifying"
                : "Verify"
              }
            </Button>
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
                ? (
                  <>
                    {!error &&
                      <>
                        <CheckCircleIcon fontSize="large"/>
                        This proof is valid for this cid!
                      </>
                    }
                    <Button
                      variant="contained"
                      onClick={onMint}
                      disabled={isMinting}
                      sx={{ marginTop: "1rem" }}
                    >
                      Mint an NFT
                    </Button>
                  </>
                )
                : (
                  <>
                    <CancelIcon fontSize="large" />
                    This proof is not valid for this cid!
                  </>
                ))
            }
            {!mintedBlock && isMinting && (
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
            }
          </Typography>
        }
      </Box>
    </Center>
  )
}