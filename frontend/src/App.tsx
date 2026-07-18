import { Route, Routes } from 'react-router-dom'
import './App.css'
import WorldRecords from './pages/WorldRecords'
import Solve from './pages/Solve'
import Puzzle from './pages/Puzzle'
import User from './pages/User'
import SignIn from './pages/SignIn'

function App() {


  return (
    <Routes>
      <Route path="/" element={<WorldRecords/>} />
      <Route path="/solve" element={<Solve/>} />
      <Route path="/puzzle" element={<Puzzle/>} />
      <Route path="/user" element={<User/>} />
      <Route path="/signin" element={<SignIn/>} />
    </Routes>
  )
}

export default App
