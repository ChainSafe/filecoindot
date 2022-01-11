import { Box, Button, TextField, Typography } from "@mui/material"
import React, { ChangeEvent, useCallback, useState } from "react"
import { Center } from "../components/layout/Center"
import { useApi } from "../contexts/ApiContext"
import CheckCircleIcon from "@mui/icons-material/CheckCircle"
import CancelIcon from "@mui/icons-material/Cancel"
import { proofJSON } from "../proof"

// type Step = "verification" | "minting"

export const MintNFT = () => {
  const [cid, setCid] = useState("")
  const [proof, setProof] = useState("")
  const { api, isApiReady } = useApi()
  const [isLoading, setIsLoading] = useState(false)
  const [isValid, setIsValid] = useState<boolean | null>(null)
  const [error, setError] = useState("")
  // const [step, setStep] = useState<Step>("verification")

  const resetState = useCallback(() => {
    setIsValid(null)
    // setStep("minting")
    setError("")
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

  const onMint = useCallback(() => {
    (api.tx as any).mint(cid, proof)
      .then((r: any) => console.log("res", r))
      .catch(console.error)
  }, [api.tx, cid, proof])

  return (
    <Center>
      <h1>Verify your cid to before minting an NFT</h1>
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          "& .MuiTextField-root": { marginBottom: "2rem", width: "30rem" }
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
              "&:first-child": { marginBottom: "1rem" }
            }}
          >
            {
              isValid
                ? (
                  <>
                    <CheckCircleIcon fontSize="large"/>
                    This proof is valid for this cid!
                    <Button
                      variant="contained"
                      onClick={onMint}
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
                )
            }

          </Typography>

        }
      </Box>
    </Center>
  )
}