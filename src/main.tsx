import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'

// Create root element with specific size
const rootElement = document.getElementById('root')!;
rootElement.style.width = '800px';
rootElement.style.height = '800px';

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
