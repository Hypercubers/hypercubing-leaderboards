import { Route, Routes } from 'react-router-dom'
import './App.css'
import WorldRecords from './pages/WorldRecords'
import Solve from './pages/Solve'
import Puzzle from './pages/Puzzle'
import Solver from './pages/Solver'
import SignIn from './pages/SignIn'
import RequestOtpDiscord from './pages/RequestOtpDiscord'

function App() {


  return (
    <Routes>
      <Route path="/" element={<WorldRecords/>} />
      <Route path="/solve" element={<Solve/>} />
      <Route path="/puzzle" element={<Puzzle/>} />
      <Route path="/solver" element={<Solver/>} />
      <Route path="/signin" element={<SignIn/>} />
      <Route path="/request-otp-discord" element={<RequestOtpDiscord/>} />
    </Routes>
  )
}

export default App
