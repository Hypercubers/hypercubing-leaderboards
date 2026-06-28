import { useState, useEffect } from 'react'
import './App.css'
import Header from './components/header'
import { getPuzzles, type Puzzle } from './lib/backend'

function App() {

  const [puzzles, setPuzzles] = useState<Puzzle[]>([])

  useEffect(() => {
    getPuzzles().then(setPuzzles)
  }, [])


  return (
    <>
      <Header/>
      {puzzles.length > 0 ? puzzles.map((puzzle) => (
        <>
          <p>{puzzle.name}</p>
          <p>{puzzle.primary_filters}</p>
        </>
      ))
    :
    <p>no puzzles loaded</p>
    }
    </>
  )
}

export default App
