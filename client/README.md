# Cortex Web

Frontend PWA do Cortex.

## Stack

- React
- Vite
- TypeScript
- PWA via `vite-plugin-pwa`

## Como rodar

```powershell
cd client
npm run dev
```

Sobe em `http://localhost:5173`. O Vite faz proxy de `/health` para o backend em `http://127.0.0.1:8080`.
