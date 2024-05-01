import React from 'react';
import { Typography, Link } from '@mui/material';

function Display({ shortenedUrl }) {
  return (
    <Typography variant="body1" style={{ marginTop: '16px' }}>
      Shortened URL: <Link href={shortenedUrl}>{shortenedUrl}</Link>
    </Typography>
  );
}

export default Display;
