import { Box, Button, TextField, Typography } from "@mui/material"
import React, { useState } from "react"
import { VerifyCid } from "../modules/VerifyCid"

type Step = "verification" | "minting"
export const MintNFT: React.FC = () => {
  const [validCid, setValidCid] = useState("")
  const [step, setStep] = useState<Step>("verification")

  return (
    <>
      {step === "verification" &&
        <VerifyCid
          onNextStep={() => setStep("minting")}
          setValidCid={setValidCid}
        />
      }
      {step === "minting" && <Minting cid={validCid}>}
    </>
  )
}
