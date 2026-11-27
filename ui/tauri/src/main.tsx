import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { FeatureWindowRoot } from './FeatureWindowRoot.tsx'
import { currentWindowLabel } from './featureWindows.ts'

const label = currentWindowLabel()
const featureKind = label === 'zitera-mentor' ? 'mentor' : label === 'zitera-burp' ? 'burp' : null

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    {featureKind ? <FeatureWindowRoot kind={featureKind} /> : <App />}
  </StrictMode>,
)
