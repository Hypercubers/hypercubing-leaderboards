import { Route, Routes } from 'react-router-dom'
import './App.css'
import WorldRecords from './pages/WorldRecords'
import Solve from './pages/Solve'

function App() {


  return (
    <Routes>
      <Route path="/" element={<WorldRecords/>} />
      <Route path="/solve" element={<Solve/>} />
    </Routes>
  )
}

export default App
