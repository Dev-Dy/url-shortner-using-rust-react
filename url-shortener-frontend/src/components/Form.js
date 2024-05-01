import React, { useState } from 'react';
import { TextField, Button } from '@mui/material';

function Form({ onSubmit }) {
  const [url, setUrl] = useState('');

  const handleChange = (event) => {
    setUrl(event.target.value);
  };

  const handleSubmit = (event) => {
    event.preventDefault();
    onSubmit(url);
  };

  return (
    <form onSubmit={handleSubmit}>
      <TextField
        label="Enter URL"
        variant="outlined"
        fullWidth
        value={url}
        onChange={handleChange}
        style={{ marginBottom: '16px' }}
      />
      <Button
        type="submit"
        variant="contained"
        color="primary"
        size="large"
        fullWidth
      >
        Shorten
      </Button>
    </form>
  );
}

export default Form;
