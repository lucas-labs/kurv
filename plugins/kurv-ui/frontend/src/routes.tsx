import { Navigate, createBrowserRouter } from 'react-router';
import { ProtectedRoute } from './components/protected-route';
import { AdminLayout } from './components/templates/admin-layout';
import { RootLayout } from './components/templates/root';
import { HomePage } from './pages/home';
import { EggDetailPage } from './pages/home/egg-detail';
import { LoginPage } from './pages/login';
import { SetupPage } from './pages/setup';

export const router = createBrowserRouter([
    {
        path: '/',
        Component: RootLayout,
        children: [
            {
                index: true,
                element: <Navigate to="/eggs" replace />,
            },
            {
                path: '/',
                handle: { crumb: 'Consola de Administración' },
                element: (
                    <ProtectedRoute>
                        <AdminLayout />
                    </ProtectedRoute>
                ),
                children: [
                    {
                        path: '/eggs',
                        Component: HomePage,
                        handle: { crumb: 'Eggs Management' },
                    },
                    {
                        path: '/eggs/:eggId',
                        Component: EggDetailPage,
                        handle: { crumb: 'Egg Detail' },
                    },
                ],
            },
            { path: 'setup', Component: SetupPage, handle: { crumb: 'Setup' } },
            { path: 'login', Component: LoginPage, handle: { crumb: 'Login' } },
            { path: '*', element: <Navigate to="/" replace /> },
        ],
    },
]);
