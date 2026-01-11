import type { Component } from 'solid-js';
import { Router, Route, Navigate } from '@solidjs/router';
import { isAuthenticated } from './stores/authStore';
import { ToastContainer } from './components/Toast';
import Login from './pages/Login';
import Feed from './pages/Feed';
import Settings from './pages/Settings';
import './index.css';

// Protected route component
const ProtectedRoute: Component<{ component: Component }> = (props) => {
    if (!isAuthenticated()) {
        return <Navigate href="/login" />;
    }
    return <props.component />;
};

// Public route (redirects if already authenticated)
const PublicRoute: Component<{ component: Component }> = (props) => {
    if (isAuthenticated()) {
        return <Navigate href="/" />;
    }
    return <props.component />;
};

const App: Component = () => {
    return (
        <>
            <ToastContainer />
            <Router>
                <Route path="/login" component={() => <PublicRoute component={Login} />} />
                <Route path="/settings" component={() => <ProtectedRoute component={Settings} />} />
                <Route path="/" component={() => <ProtectedRoute component={Feed} />} />
            </Router>
        </>
    );
};

export default App;
