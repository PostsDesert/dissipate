import type { Component } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { LoginForm } from '../components/LoginForm';
import { ThemeToggle } from '../components/ThemeToggle';
import './Login.css';

export const Login: Component = () => {
    const navigate = useNavigate();

    const handleLoginSuccess = () => {
        navigate('/', { replace: true });
    };

    return (
        <div class="login-page">
            <div class="login-header">
                <ThemeToggle />
            </div>
            <main class="login-main">
                <LoginForm onSuccess={handleLoginSuccess} />
            </main>
        </div>
    );
};

export default Login;
