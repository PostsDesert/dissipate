import { Router } from '@solidjs/router'
import Login from './pages/Login'
import Feed from './pages/Feed'
import Settings from './pages/Settings'

function App() {
  return (
    <Router>
      <Login path="/" />
      <Feed path="/feed" />
      <Settings path="/settings" />
    </Router>
  )
}

export default App
