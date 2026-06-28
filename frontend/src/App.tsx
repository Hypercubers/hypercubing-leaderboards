import { Route, Routes } from 'react-router-dom'
import './App.css'
import WorldRecords from './pages/WorldRecords'

function App() {


  return (
    <Routes>
      <Route path="/" element={<WorldRecords/>} />
    </Routes>
  )
}

export default App
