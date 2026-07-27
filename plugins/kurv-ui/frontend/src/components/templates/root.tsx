import { Navigate, Outlet, useLocation } from 'react-router';
import { type ReactNode, useContext } from 'react';
import { SetupStatusContext } from '@/contexts/setup/context';
import { SetupStatusProvider } from '@/contexts/setup/provider';
import { ThemeProvider } from '@/contexts/theme/provider';
import { TooltipProvider } from '../ui/tooltip';

const Root = ({ children }: { children?: ReactNode }) => {
    const { status, error } = useContext(SetupStatusContext);
    const location = useLocation();

    if (status === 'loading') return null;
    if (status === 'errored')
        return (
            <FatalErrorPage
                label={'Error obteniendo estado del server: '}
                error={error || 'An unknown error occurred.'}
            />
        );

    if (status === 'uninitialized') {
        // if we are not already on the setup page, redirect to it
        if (location.pathname !== '/setup') {
            return <Navigate to="/setup" replace />;
        }
    }

    return <>{children}</>;
};

export function FatalErrorPage({ error, label }: { error: string; label: string }) {
    return (
        <section className="flex h-screen flex-col items-center justify-center font-mono tracking-tighter">
            <div className="max-w-2xl p-4">
                <h1 className="px-4">Error</h1>
                <p className="px-4">
                    <span className="text-muted-foreground">{label}</span>
                    <span className="text-foreground">{error}</span>
                    <br />
                </p>
                <p className="text-muted-foreground mt-2 px-4">
                    Check the console for more details.
                </p>
            </div>
        </section>
    );
}

export function RootLayout() {
    return (
        <SetupStatusProvider>
            <ThemeProvider>
                <TooltipProvider>
                    <Root>
                        <Outlet />
                    </Root>
                </TooltipProvider>
            </ThemeProvider>
        </SetupStatusProvider>
    );
}
