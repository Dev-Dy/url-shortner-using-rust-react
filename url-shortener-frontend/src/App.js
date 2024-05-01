import React, { useState } from 'react';
import { Container, Typography } from '@mui/material';
import { createTheme, ThemeProvider } from '@mui/material/styles';
import Form from './components/Form';
import Display from './components/Display';
import './App.css';

const theme = createTheme();

function App() {
  const [shortenedUrl, setShortenedUrl] = useState('');

  const handleSubmit = async (url) => {
    try {
      const response = await fetch('http://localhost:8080/shorten', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ url }),
      });

      if (response.ok) {
        const data = await response.json();
        setShortenedUrl(`http://localhost:8080/${data}`);
      } else {
        console.error('Failed to shorten URL');
      }
    } catch (error) {
      console.error('Error:', error);
    }
  };

  return (
    <ThemeProvider theme={theme}>
      <Container maxWidth="sm" className="App">
        <Typography variant="h2" gutterBottom>
          URL Shortener
        </Typography>
        <Form onSubmit={handleSubmit} />
        {shortenedUrl && <Display shortenedUrl={shortenedUrl} />}
      </Container>
    </ThemeProvider>
  );
}

export default App;
