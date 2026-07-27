import { createRoot } from 'react-dom/client';
import { RouterProvider } from 'react-router';
import { StrictMode } from 'react';
import { AuthProvider } from './contexts/auth/provider';
import { router } from './routes';
import './main.css';

const bootstrap = async () => {
    const root = createRoot(document.getElementById('app') as HTMLElement);

    /** The main app */
    const App = () => {
        return (
            <StrictMode>
                <AuthProvider>
                    <RouterProvider router={router} />
                </AuthProvider>
            </StrictMode>
        );
    };

    root.render(<App />);
};

bootstrap().catch((error) => {
    console.error('Failed to bootstrap the application:', error);
});
