import AppBar from "@mui/material/AppBar"
import React, { useEffect } from "react"
import { Avatar, Box, Button, Container, IconButton, Toolbar, Tooltip, Typography } from "@mui/material"
import { AccountSelect } from "../AccountSelect"
import { Identicon } from "@polkadot/react-identicon"
import { Link } from "react-router-dom"
import { useAccountList } from "../../contexts/AccountsContext"
import { useApi } from "../../contexts/ApiContext"

export const Header: React.FC = () => {
  const { selected } = useAccountList()
  const [anchorElUser, setAnchorElUser] = React.useState<null | HTMLElement>(null)
  const { api, isApiReady } = useApi()
  const handleOpenUserMenu = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorElUser(event.currentTarget)
  }

  useEffect(() => {
    if (!isApiReady) return

    let unsubscribe: () => void

    // use the api
    api.derive.chain.bestNumber((number) => {
      console.log(number.toNumber())
    })
      .then(unsub => {unsubscribe = unsub})
      .catch(console.error)

    return () => unsubscribe && unsubscribe()
  })

  const handleCloseUserMenu = () => {
    setAnchorElUser(null)
  }

  return (
    <AppBar>
      <Container maxWidth="xl">
        <Toolbar disableGutters>
          <Typography
            variant="h6"
            noWrap
            component="div"
            sx={{ flexGrow: 1 }}
          >
            Filecoindot pallet demo
          </Typography>
          {selected && (
            <>
              <Box sx={{ flexGrow: 1, display: "flex" }}>
                <Button
                  component={Link}
                  to="/"
                  sx={{ my: 2, color: "white", display: "block" }}
                >
                  Verify cid
                </Button>
                {/* <Button
                  component={Link}
                  to="/"
                  sx={{ my: 2, color: "white", display: "block" }}>
                  Show my NFTs
                </Button> */}
              </Box>
              <Box sx={{ flexGrow: 0 }}>
                <Tooltip title="Select Account">
                  <IconButton
                    onClick={handleOpenUserMenu}
                    sx={{ p: 0 }}>
                    <Avatar sx={{ bgcolor: "background.paper" }}>
                      <Identicon
                        value={selected}
                        theme="substrate"
                        size={32} />
                    </Avatar>
                  </IconButton>
                </Tooltip>
                <AccountSelect
                  anchorEl={anchorElUser}
                  onClose={handleCloseUserMenu}
                  PaperProps={{
                    elevation: 0,
                    sx: {
                      filter: "drop-shadow(0px 2px 8px rgba(0,0,0,0.32))",
                      mt: 1.5,
                      maxHeight: "100%",
                      overflow: "auto",
                      "& .MuiAvatar-root": {
                        width: 32,
                        height: 32,
                        ml: -0.5,
                        mr: 1
                      },
                      "&:before": {
                        content: "\"\"",
                        display: "block",
                        position: "absolute",
                        top: 0,
                        right: 16,
                        width: 10,
                        height: 10,
                        bgcolor: "background.paper",
                        transform: "translateY(-50%) rotate(45deg)",
                        zIndex: 0
                      }
                    }
                  }}
                  transformOrigin={{ horizontal: "right", vertical: "top" }}
                  anchorOrigin={{ horizontal: "right", vertical: "bottom" }}
                />
              </Box>
            </>
          )}
        </Toolbar>
      </Container>
    </AppBar>
  )
}
