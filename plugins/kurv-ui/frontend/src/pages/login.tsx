import { Navigate, useNavigate } from 'react-router';
import { toast } from 'sonner';
import { useState } from 'react';
import { LoginForm } from '@/components/blocks/login-form';
import { KurvLogo } from '@/components/ui/kurv-logo';
import { Toaster } from '@/components/ui/toaster';
import { useApi } from '../hooks/use-api/hook';
import { useAuth } from '../hooks/use-auth';

export function LoginPage() {
    const [isLoading, setIsLoading] = useState(false);
    const { isAuthenticated, login } = useAuth();

    const api = useApi((api) => api.auth);
    const navigate = useNavigate();

    // redirect to home if already authenticated
    if (isAuthenticated) {
        return <Navigate to="/" replace />;
    }

    const handleSubmit = async (username: string, password: string) => {
        setIsLoading(true);

        try {
            const response = await api.login({ username, password }).json();
            login(response.accessToken);
            navigate('/', { replace: true });
        } catch (err: unknown) {
            const error = err as { message?: string };
            console.error('Login failed:', error);
            toast.error(
                'Error al iniciar sesión. Por favor, verifica tus credenciales e inténtalo de nuevo.',
            );
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <>
            <div className="flex min-h-svh flex-col items-center justify-center px-4">
                <div className="flex w-full max-w-xs flex-col items-center gap-10">
                    <KurvLogo className="h-16 w-auto" />

                    <LoginForm className="w-full" onSubmit={handleSubmit} isLoading={isLoading} />
                </div>
            </div>

            <Toaster />
        </>
    );
}
