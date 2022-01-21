/* eslint-disable max-len */
import axios from "axios"
import { useCallback, useEffect, useState } from "react"

const COLLECTION_METADATA_BUCKET_NAME = "filecoindot-collection-metadata"
const NFT_METADATA_BUCKET_NAME = "filecoindot-nft-metadata"
const NFT_IMAGES_BUCKET_NAME = "filecoindot-nft-images"

const STORAGE_API_TOKEN = process.env.REACT_APP_STORAGE_TOKEN || "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE2NDI1OTg5NTgsImNuZiI6eyJqa3UiOiIvY2VydHMiLCJraWQiOiI5aHE4bnlVUWdMb29ER2l6VnI5SEJtOFIxVEwxS0JKSFlNRUtTRXh4eGtLcCJ9LCJ0eXBlIjoiYXBpX3NlY3JldCIsImlkIjo2MywidXVpZCI6IjU2Zjk3ZTAyLTUzMjctNDI5Ni1iMDllLTVkMmY0NDc3NDdjZCIsInBlcm0iOnsiYmlsbGluZyI6IioiLCJzZWFyY2giOiIqIiwic3RvcmFnZSI6IioiLCJ1c2VyIjoiKiJ9LCJhcGlfa2V5IjoiWFBQSk9RRUFZR1BXUkpBT05NWFYiLCJzZXJ2aWNlIjoic3RvcmFnZSJ9.VjOxvoPRHWFKkD60n5QH2v8O1w0Y2x1c4cKfg_1WhJufVwCFPNa4U_yBgG9-butWU1igZqDkmuGFDKwN0JVbNg"
const API = "https://stage.imploy.site/api/v1"

const useBuckets = () => {
  const [collectionMetaBucketId, setCollectionMetaBucketId] = useState("")
  const [nftMetaBucketId, setNftMetaBucketId] = useState("")
  const [nftImagesBucketId, setNftImagesBucketId] = useState("")

  const getBuckets = useCallback(() => {
    return axios.get<Record<string, any>[]>(
      `${API}/buckets`,
      { headers: {
        "Authorization" : `Bearer ${STORAGE_API_TOKEN}`
      } }
    )
  }, [])

  const createBucket = useCallback((bucketName: string, setter: React.Dispatch<React.SetStateAction<string>>) => {
    const bucketCreationBody = {
      type: "fps",
      name: bucketName
    }

    axios.post(
      `${API}/buckets`,
      bucketCreationBody,
      { headers: {
        "Authorization" : `Bearer ${STORAGE_API_TOKEN}`
      } }
    ).then(({ data }) => {
      setter(data.id)
    })
      .catch(console.error)
  }, [])

  const uploadFile = useCallback((file: File, bucketId: string) => {
    if(!bucketId) return Promise.reject("no bucket id")

    // Create an object of formData
    const formData = new FormData()

    // Update the formData object
    formData.append(
      "file",
      file,
      file.name
    )

    formData.append("path", "/")

    return axios.post(
      `${API}/bucket/${bucketId}/upload`,
      formData,
      { headers: {
        "Authorization" : `Bearer ${STORAGE_API_TOKEN}`
      } }
    ).then(async ({ data }) => {
      console.log("data", data)
      const fileInfo = await axios.post<{content: {cid: string}}>(
        `${API}/bucket/${bucketId}/file`,
        {
          path: `${file.name}`
        },
        { headers: {
          "Authorization" : `Bearer ${STORAGE_API_TOKEN}`
        } }
      )

      return fileInfo.data.content.cid
    })
      .catch((e) => {
        console.error(e)
        return Promise.reject(e)
      })
  }, [])

  const setOrCreateBucket = useCallback((bucketName: string, setter: React.Dispatch<React.SetStateAction<string>>) => {
    getBuckets()
      .then(({ data }) => {
        const bucketToFind = data.find((bucket) => bucket.name === bucketName)

        bucketToFind
          ? setter(bucketToFind.id)
          : createBucket(bucketName, setter)
      })
      .catch(console.error)

  }, [createBucket, getBuckets])

  useEffect(() => {
    if(!collectionMetaBucketId) setOrCreateBucket(COLLECTION_METADATA_BUCKET_NAME, setCollectionMetaBucketId)

    if(!nftMetaBucketId) setOrCreateBucket(NFT_METADATA_BUCKET_NAME, setNftMetaBucketId)

    if(!nftImagesBucketId) setOrCreateBucket(NFT_IMAGES_BUCKET_NAME, setNftImagesBucketId)
  }, [collectionMetaBucketId, nftImagesBucketId, nftMetaBucketId, setOrCreateBucket])


  return { collectionMetaBucketId, nftImagesBucketId, nftMetaBucketId, uploadFile }
}

export default useBuckets