import { Route, Routes } from 'react-router-dom'
import './App.css'
import WorldRecords from './pages/WorldRecords'
import Solve from './pages/Solve'
import Puzzle from './pages/Puzzle'
import Solver from './pages/Solver'
import SignIn from './pages/SignIn'
import RequestOtpDiscord from './pages/RequestOtpDiscord'
import SubmitSolve from './pages/SubmitSolve'
import EditSolve from './pages/EditSolve'
import MySubmissions from './pages/MySubmissions'
import Settings from './pages/Settings'
import Error from './pages/Error'
import RequireAuth from './components/require-auth'
import RequireModerator from './components/require-moderator'
import AuditLog from './pages/moderator/AuditLog'

function App() {


  return (
    <Routes>
      <Route path="/" element={<WorldRecords/>} />
      <Route path="/solve" element={<Solve/>} />
      <Route path="/puzzle" element={<Puzzle/>} />
      <Route path="/solver" element={<Solver/>} />
      <Route path="/signin" element={<SignIn/>} />
      <Route path="/request-otp-discord" element={<RequestOtpDiscord/>} />
      <Route element={<RequireAuth/>}>
        <Route path="/submit-solve" element={<SubmitSolve/>} />
        <Route path="/edit-solve" element={<EditSolve/>} />
        <Route path="/my-submissions" element={<MySubmissions/>} />
        <Route path="/settings" element={<Settings/>} />
      </Route>
      <Route element={<RequireModerator/>}>
        <Route path="/audit-log/general" element={<AuditLog/>} />
      </Route>
      {/* Wildcard that redirects unknown URLS to 404 page */}
      <Route path="*" element={<Error code="404" msg="Not Found. The page you are looking for does not exist."/>} />
    </Routes>
  )
}

export default App
