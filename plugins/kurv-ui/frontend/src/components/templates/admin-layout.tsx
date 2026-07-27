import { LogOut } from 'lucide-react';
import { NavLink, Outlet, useNavigate } from 'react-router';
import { Toaster, toast } from 'sonner';
import { IconButton } from '@/components/blocks/icon-button';
import { useAuth } from '@/hooks/use-auth';
import { KurvLogo } from '../ui/kurv-logo';

export function AdminLayout() {
    const navigate = useNavigate();
    const { logout, sub } = useAuth();

    const handleLogout = async () => {
        try {
            await logout();
            navigate('/login', { replace: true });
        } catch (error) {
            console.error('Logout failed:', error);
            toast.error('Error al cerrar sesión. Por favor, inténtalo de nuevo.');
        }
    };

    return (
        <>
            <main className="relative flex min-h-svh flex-1 flex-col">
                <header className="sticky top-0 z-10">
                    <div className="bg-background/80 backdrop-blur-xl">
                        <div className="mx-auto flex h-16 max-w-6xl items-center px-6">
                            <nav className="flex flex-1 items-center gap-5">
                                <NavLink
                                    to="/eggs"
                                    className="text-muted-foreground hover:text-foreground text-sm transition-colors"
                                >
                                    Processes
                                </NavLink>
                            </nav>

                            <a href="/eggs" className="absolute left-1/2 -translate-x-1/2">
                                <KurvLogo className="h-7" />
                            </a>

                            <div className="flex flex-1 items-center justify-end gap-3">
                                <span className="text-muted-foreground text-xs tracking-wide">
                                    {sub || 'admin'}
                                </span>
                                <span className="bg-border h-3 w-px" />
                                <IconButton
                                    icon={LogOut}
                                    tooltip="Salir"
                                    onClick={() => void handleLogout()}
                                />
                            </div>
                        </div>
                    </div>
                    <div className="via-border/80 h-px bg-linear-to-r from-transparent to-transparent" />
                </header>

                <div className="mx-auto w-full max-w-6xl flex-1 px-6 py-8">
                    <Outlet />
                </div>

                <footer className="mt-auto">
                    <div className="via-border/80 h-px bg-linear-to-r from-transparent to-transparent" />
                    <div className="mx-auto flex h-10 max-w-6xl items-center justify-center px-6">
                        <span className="text-muted-foreground text-xs">
                            Made with 🧉 by{' '}
                            <a
                                href="https://github.com/lucas-labs"
                                target="_blank"
                                rel="noopener noreferrer"
                                className="hover:text-foreground transition-colors"
                            >
                                lucode
                            </a>
                        </span>
                    </div>
                </footer>
            </main>
            <Toaster />
        </>
    );
}
