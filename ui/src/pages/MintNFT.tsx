import { Card, CardContent, CardMedia, Grid, TextField, Typography } from "@mui/material"
import { makeStyles } from "@mui/styles"
// import axios from "axios"
import React, { ChangeEvent, useCallback, useState } from "react"
import { Center } from "../components/layout/Center"
import { useNftList } from "../contexts/NftContext"
import useBuckets from "../hooks/useBuckets"
import { DropzoneArea } from "mui-file-dropzone"
import LoadingButton from "@mui/lab/LoadingButton"
import useNft from "../hooks/usNft"
import SendIcon from "@mui/icons-material/Send"
import CheckCircleIcon from "@mui/icons-material/CheckCircle"

export const MintNFT: React.FC = () => {
  const useStyles = makeStyles({
    root: {
      padding: "1rem",
      minHeight: "150px"
    }
  })

  const classes = useStyles()
  const { uploadFile, nftImagesBucketId } = useBuckets()
  const [selectedFile, setSelectedFile] = useState<File | undefined>()
  const { nftList, addNft } = useNftList()
  const [name, setName] = useState("")
  const [isLoading, setIsLoading] = useState(false)
  const [status, setStatus] = useState<"init" | "creating collection" | "uploading" | "minting">("init")
  const [copiedCid, setCopiedCid] = useState("")
  const { createCollection, isCollectionCreated, mintNft } = useNft()
  const [mintedAtBlock, setMintedAtBlock] = useState("")
  const [error, setError] = useState("")

  const onFileSelected = useCallback((fileList: File[]) => {

    const fileToUpload = fileList[0]

    if(!fileToUpload) return

    if(!fileToUpload.type.startsWith("image/")) {

      console.error("Not an image", fileToUpload)
      return
    }

    setSelectedFile(fileToUpload)
  }, [])

  const onFileUpload = async () => {
    if (!selectedFile) return

    setIsLoading(true)
    setStatus("uploading")

    uploadFile(selectedFile, nftImagesBucketId)
      .then(async (cid) => {

        if(!isCollectionCreated) {
          setStatus("creating collection")
          await createCollection()
        }

        setStatus("minting")

        const blockId = await mintNft(cid, name)

        if(blockId) {
          setMintedAtBlock(blockId)
          setTimeout(() => setMintedAtBlock(""), 5000)
        }


        addNft({
          cid,
          filePreview: URL.createObjectURL(selectedFile),
          name
        })
      })
      .catch((e: any) => {
        setError(e.message)
        setTimeout(() => setError(""), 5000)
        console.error(e)
      })
      .finally(() => {
        setIsLoading(false)
        setName("")
        setSelectedFile(undefined)
        setStatus("init")
      })
  }

  const onNameChange = useCallback((cid: ChangeEvent<HTMLInputElement>) => {
    setName(cid.currentTarget.value)
    //reset the flag
    !!mintedAtBlock && setMintedAtBlock("")
  }, [mintedAtBlock])

  const onCopyCid = useCallback(async (cid: string) => {
    await navigator.clipboard.writeText(cid)
    setCopiedCid(cid)
    setTimeout(() => setCopiedCid(""), 3000)
  }, [])

  // const sendRpc = useCallback(() => {
  //   axios.post(
  //     "http://127.0.0.1:5001",
  //     { "jsonrpc":"2.0", "id":"0", "method":"net_discover", "params":{ "searchTime":3 } }
  //   ).then(console.log)
  //     .catch(console.error)
  // }, [])

  return (
    <>
      <Center>
        <h1>Mint a RMRK NFT</h1>
        <TextField
          fullWidth
          required
          id="name"
          label="Name"
          placeholder="My awesome NFT"
          onChange={onNameChange}
          value={name}
          sx={{ marginBottom: "1rem" }}
        />
        <div>
          <DropzoneArea
            filesLimit={1}
            acceptedFiles={["image/*"]}
            showPreviews={false}
            maxFileSize={5000000}
            fileObjects={selectedFile}
            onChange={onFileSelected}
            dropzoneClass={classes.root}
            showPreviewsInDropzone={!!selectedFile}
            showAlerts={false}
          />
        </div>
        <LoadingButton
          variant="contained"
          onClick={onFileUpload}
          disabled={!name || !selectedFile || status !== "init"}
          loading={isLoading}
          loadingPosition="start"
          sx={{ marginTop: "1rem", width: "100%" }}
          startIcon={<SendIcon />}
        >
          {isLoading
            ? status
            : "Let's go"
          }
        </LoadingButton>
        { !!mintedAtBlock &&
          <Typography
            variant="h6"
            noWrap
            component="div"
            sx={{
              marginTop: "1rem",
              color: "lightseagreen",
              display: "flex",
              alignContent: "center",
              justifyContent: "center",
              alignItems: "center",
              flexDirection: "column",
              "&:first-of-type": { marginBottom: "1rem" },
              maxWidth: "400px",
              wordWrap: "normal",
              textAlign: "center",
              whiteSpace: "unset",
              overflow: "visible"
            }}
          >
            <>
              <CheckCircleIcon fontSize="large"/>
              Nft minted at block {mintedAtBlock}
            </>
          </Typography>
        }
        {error && (
          <Typography
            variant="h6"
            noWrap
            component="div"
            sx={{
              marginTop: "1rem",
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
        {/* <Button
          variant="contained"
          component="span"
          onClick={sendRpc}
        >
          send rpc
        </Button> */}
      </Center>
      {!!nftList.length && <h1>Your freshly minted NFTs:</h1>}
      <Grid
        container
        spacing={{ xs: 3 }}
        columns={{ xs: 8 }}
      >
        {nftList.map((nft, index) => (
          <Grid
            item
            xs={2}
            key={index}
            // onClick={handleClick(index)}
          >
            <Card
              sx={{ maxWidth: 345 }}
              // raised={index === selected}
            >
              <CardMedia
                component="img"
                alt="green iguana"
                height="200"
                width="200"
                image={nft.filePreview}
              />
              <CardContent>
                <Typography
                  gutterBottom
                  variant="h5"
                  component="div"
                  sx={{
                    textOverflow: "ellipsis",
                    overflow: "hidden"
                  }}
                >
                  {nft.name}
                </Typography>
                <Typography
                  variant="body2"
                  color="text.secondary"
                  sx={{
                    textOverflow: "ellipsis",
                    overflow: "hidden",
                    cursor: "pointer"
                  }}
                  onClick={() => onCopyCid(nft.cid)}
                >
                  {nft.cid === copiedCid
                    ? "copied!"
                    : nft.cid
                  }
                </Typography>
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>
    </>
  )
}
