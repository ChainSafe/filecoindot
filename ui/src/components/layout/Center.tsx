import React from "react"
import { Grid } from "@mui/material"

export const Center: React.FC = ({ children }) => (
  <Grid
    container
    spacing={0}
    direction="column"
    alignItems="center"
    justifyContent="center"
    style={{ height: "100%" }}>
    <Grid
      item
      xs={3}>
      {children}
    </Grid>
  </Grid>
)
