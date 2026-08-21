import { Route, Routes } from 'react-router-dom'
import './App.css'
import WorldRecords from './pages/WorldRecords'
import Solve from './pages/Solve'
import Puzzle from './pages/Puzzle'
import Solver from './pages/Solver'
import SignIn from './pages/SignIn'
import RequestOtpDiscord from './pages/RequestOtpDiscord'
import SubmitSolve from './pages/SubmitSolve'
import MySubmissions from './pages/MySubmissions'
import Settings from './pages/Settings'
import FourZeroFour from './pages/FourZeroFour'

function App() {


  return (
    <Routes>
      <Route path="/" element={<WorldRecords/>} />
      <Route path="/solve" element={<Solve/>} />
      <Route path="/puzzle" element={<Puzzle/>} />
      <Route path="/solver" element={<Solver/>} />
      <Route path="/signin" element={<SignIn/>} />
      <Route path="/request-otp-discord" element={<RequestOtpDiscord/>} />
      <Route path="/submit-solve" element={<SubmitSolve/>} />
      <Route path="/my-submissions" element={<MySubmissions/>} />
      <Route path="/settings" element={<Settings/>} />
      {/* Wildcard that redirects unknown URLS to 404 page */}
      <Route path="*" element={<FourZeroFour/>} />
    </Routes>
  )
}

export default App
