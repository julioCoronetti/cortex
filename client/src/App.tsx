import { useEffect, useState } from 'react'
import './App.css'

function App() {
  const [status, setStatus] = useState<string>('checking...')

  useEffect(() => {
    fetch('/health')
      .then((r) => r.json())
      .then((d) => setStatus(d.status === 'ok' ? 'online' : 'unknown'))
      .catch(() => setStatus('offline'))
  }, [])

  return (
    <main>
      <h1>Cortex</h1>
      <p>Backend: {status}</p>
    </main>
  )
}

export default App
