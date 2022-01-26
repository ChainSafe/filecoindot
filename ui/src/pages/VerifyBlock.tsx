import { Box, CircularProgress, Container, TextField, Typography } from "@mui/material"
import LoadingButton from "@mui/lab/LoadingButton"
import React, { ChangeEvent, useCallback, useEffect, useState } from "react"
import { Center } from "../components/layout/Center"
import { useApi } from "../contexts/ApiContext"
import CheckCircleIcon from "@mui/icons-material/CheckCircle"
import CancelIcon from "@mui/icons-material/Cancel"
import substrateImage from "../logos/substrate.png"
import filecoinImage from "../logos/Filecoin.png"
import chainSafeImage from "../logos/chainsafe.png"
import rmrkImage from "../logos/rmrk.png"

export const VerifyBlock = () => {
  const [cid, setCid] = useState("")
  const { api, isApiReady } = useApi()
  const [isLoading, setIsLoading] = useState(false)
  const [isValid, setIsValid] = useState<boolean | null>(null)
  const [error, setError] = useState("")
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
    setCid(cid.currentTarget.value.trim())
    resetState()
  }, [resetState])

  const refreshCidMap = useCallback(() =>
    api.query.filecoindot.verifiedBlocks.entries()
      .then((res) => {
        const arr = res.map(([res1]) => (res1.toHuman() as string[])[0])
        setCidMap(arr)
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

  return (
    <>
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
            </Typography>
          }
        </Box>
      </Center>
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          position: "fixed",
          bottom: "1rem",
          left: 0,
          width: "100%",
          "& .MuiTextField-root": { marginBottom: "2rem" }
        }}
      >
        <Typography
          variant="h6"
          noWrap
          component="div"
          sx={{ textAlign: "center", marginBottom: "1rem" }}
        >
          Powered by
        </Typography>
        <Container
          sx={{
            display: "flex",
            alignItems: "center",
            height: "5rem",
            maxWidth: "650px !important"
          }}>
          <Container
            sx={{
              textAlign: "center",
              "& img": {
                maxWidth: "8rem",
                height: "auto"
              }
            }}
          >
            <a
              href="https://docs.substrate.io/"
              target="_blank"
              rel="noreferrer"><img src={substrateImage}/></a>
          </Container>
          <Container
            sx={{
              height: "100%",
              textAlign: "center",
              "& img": {
                height: "100%"
              } }}
          >
            <a
              href="https://filecoin.io/"
              target="_blank"
              rel="noreferrer"><img src={filecoinImage}/></a>
          </Container>
          <Container
            sx={{
              height: "100%",
              textAlign: "center",
              "& img": {
                height: "100%"
              }
            }}
          >
            <a
              href="https://storage.chainsafe.io/"
              target="_blank"
              rel="noreferrer"><img src={chainSafeImage}/></a>
          </Container>
          <Container sx={{
            textAlign: "center",
            "& img": {
              height: "auto",
              maxWidth: "8rem"
            }
          }}>
            <a
              href="https://github.com/rmrk-team/rmrk-spec"
              target="_blank"
              rel="noreferrer"><img src={rmrkImage}/></a>
          </Container>
        </Container>
      </Box>
    </>
  )
}