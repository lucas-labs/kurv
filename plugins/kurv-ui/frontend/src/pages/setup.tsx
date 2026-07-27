import { Navigate, useNavigate } from 'react-router';
import { toast } from 'sonner';
import { use, useState } from 'react';
import { UserPasswordForm } from '@/components/blocks/user-password-form';
import { KurvLogo } from '@/components/ui/kurv-logo';
import { Toaster } from '@/components/ui/toaster';
import { SetupStatusContext } from '@/contexts/setup/context';
import { useApi } from '../hooks/use-api/hook';

export function SetupPage() {
    const [isLoading, setIsLoading] = useState(false);
    const api = useApi((api) => api.admin.setup);
    const { status, setStatus } = use(SetupStatusContext);
    const navigate = useNavigate();

    // Redirect to home if already set up
    if (status === 'ready') {
        return <Navigate to="/" replace />;
    }

    const handleSubmit = async (username: string, password: string) => {
        setIsLoading(true);

        try {
            const response = await api.initialize({ username, password }).json();

            toast.success(
                `Setup completado exitosamente! Ingresa con el usuario: ${response.username} para continuar.`,
            );

            setStatus?.('ready');
            await navigate('/login', { replace: false });
        } catch (err: unknown) {
            const error = err as { message?: string };
            console.error('Error during setup:', error);

            toast.error(
                error.message || 'Error al completar el setup. Por favor, intenta nuevamente.',
            );
            setStatus?.('errored');
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <>
            <div className="flex min-h-svh flex-col items-center justify-center gap-8 p-4 md:gap-16 md:p-6">
                <KurvLogo className="h-8" />

                <div className="w-full max-w-sm md:max-w-sm">
                    <UserPasswordForm
                        title="Bienvenido!"
                        description="Por favor, completá el setup inicial. Ingresá un usuario y contraseña para el administrador."
                        onSubmit={handleSubmit}
                        isLoading={isLoading}
                    />
                </div>
            </div>

            <Toaster />
        </>
    );
}
