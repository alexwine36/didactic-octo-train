import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { plus100, sum, TakeoffEngine } from 'local-bindings'
import { Button } from './components/ui/button'
import { Canvas } from './canvas'
function App() {
  const [count, setCount] = useState(0)

  console.log(sum(15, 2))
  console.log(plus100(15))
  const engine = new TakeoffEngine({
    scale: 1.0,
    offsetX: 0.0,
    offsetY: 0.0,
  })
  console.log(engine.screenToWorld({ x: 0, y: 0 }))


  return (
    <>
      <Canvas />
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <Button
          variant="outline"
          onClick={() => setCount((count) => count + 1)}
        >
          count is {count}
        </Button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
