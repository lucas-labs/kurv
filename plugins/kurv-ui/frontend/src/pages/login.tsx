import { HTTPError } from 'ky';
import { Navigate, useNavigate } from 'react-router';
import { toast } from 'sonner';
import { useState } from 'react';
import { LoginForm } from '@/components/blocks/login-form';
import { KurvLogo } from '@/components/ui/kurv-logo';
import { Toaster } from '@/components/ui/toaster';
import { useAuth } from '../hooks/use-auth';

export function LoginPage() {
    const [isLoading, setIsLoading] = useState(false);
    const [errorMessage, setErrorMessage] = useState<string | null>(null);
    const { isAuthenticated, isLoading: isAuthLoading, login } = useAuth();
    const navigate = useNavigate();

    if (isAuthLoading) {
        return null;
    }

    // redirect to home if already authenticated
    if (isAuthenticated) {
        return <Navigate to="/" replace />;
    }

    const handleSubmit = async (username: string, password: string) => {
        setIsLoading(true);
        setErrorMessage(null);

        try {
            await login(username, password);
            navigate('/', { replace: true });
        } catch (err: unknown) {
            console.error('Login failed:', err);

            const message =
                err instanceof HTTPError && err.response.status === 401
                    ? 'Usuario o contraseña inválidos.'
                    : 'No se pudo iniciar sesión. Inténtalo de nuevo.';

            setErrorMessage(message);
            toast.error(message);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <>
            <div className="flex min-h-svh flex-col items-center justify-center px-4">
                <div className="flex w-full max-w-xs flex-col items-center gap-10">
                    <KurvLogo className="h-16 w-auto" />

                    <LoginForm
                        className="w-full"
                        onSubmit={handleSubmit}
                        isLoading={isLoading}
                        errorMessage={errorMessage}
                    />
                </div>
            </div>

            <Toaster />
        </>
    );
}
