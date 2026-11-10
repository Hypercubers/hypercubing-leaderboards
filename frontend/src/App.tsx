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
import GeneralAuditLog from './pages/moderator/GeneralAuditLog'
import UserAuditLog from './pages/moderator/UserAuditLog'
import Users from './pages/moderator/Users'
import PendingSubmissions from './pages/moderator/PendingSubmissions'
import SolveAuditLog from './pages/moderator/SolveAuditLog'
import SolverSubmissions from './pages/moderator/SolverSubmissions'

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
        <Route path="/audit-log/general" element={<GeneralAuditLog/>} />
        <Route path="/audit-log/user" element={<UserAuditLog/>} />
        <Route path="/audit-log/solve" element={<SolveAuditLog/>} />
        <Route path="/users" element={<Users/>} />
        <Route path ="/pending-submissions" element={<PendingSubmissions/>} />
        <Route path="/solver-submissions" element={<SolverSubmissions/>} />
      </Route>
      {/* Wildcard that redirects unknown URLS to 404 page */}
      <Route path="*" element={<Error code="404" msg="Not Found. The page you are looking for does not exist."/>} />
    </Routes>
  )
}

export default App
