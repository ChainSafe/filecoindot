import {Card, CardContent, CardMedia, Grid, Typography} from "@mui/material";
import React, {useState} from "react";

export const Home: React.FC = () => {
    const [selected, setSelected] = useState(-1);
    const handleClick = (index: number) => () => {
        if (selected === index) setSelected(-1);
        else setSelected(index);
    }

    return (
        <Grid container spacing={{ xs: 3 }} columns={{ xs: 8 }} >
            {Array.from(Array(9)).map((_, index) => (
                <Grid item xs={2} key={index} onClick={handleClick(index)}>
                    <Card sx={{ maxWidth: 345 }} raised={index === selected}>
                        <CardMedia
                            component="img"
                            alt="green iguana"
                            height="200"
                            width="200"
                            image="http://placekitten.com/g/200/200"
                        />
                        <CardContent>
                            <Typography variant="subtitle2" color="text.secondary">
                                f24f3asdfsdvsdv
                            </Typography>
                            <Typography gutterBottom variant="h5" component="div">
                                NFT name
                            </Typography>
                            <Typography variant="body2" color="text.secondary">
                                zb2rhe5P4gXftAwvA4eXQ5HJwsER2owDyS9sKaQRRVQPn93bA
                            </Typography>
                        </CardContent>
                    </Card>
                </Grid>
            ))}
        </Grid>
    );
}
