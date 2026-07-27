import { HTTPError } from 'ky';
import { type ReactNode, useEffect, useEffectEvent, useState } from 'react';
import { useApi } from '@/hooks/use-api/hook';
import type { AuthenticatedUser } from '@/hooks/use-api/types';
import { AuthContext, type AuthContextType } from './context';

const isUnauthorized = (error: unknown) => {
    return error instanceof HTTPError && error.response.status === 401;
};

export const AuthProvider = ({ children }: { children: ReactNode }) => {
    const api = useApi((client) => client.auth);
    const [user, setUser] = useState<AuthenticatedUser | null>(null);
    const [isLoading, setIsLoading] = useState(true);

    const loadCurrentUser = useEffectEvent(async () => {
        try {
            return await api.me().json();
        } catch (error) {
            if (isUnauthorized(error)) {
                return null;
            }

            throw error;
        }
    });

    const refresh = useEffectEvent(async () => {
        setIsLoading(true);

        try {
            setUser(await loadCurrentUser());
        } finally {
            setIsLoading(false);
        }
    });

    const login = async (username: string, password: string) => {
        await api.login({ username, password }).json();
        setUser(await loadCurrentUser());
    };

    const logout = async () => {
        try {
            await api.logout();
            setUser(null);
        } catch (error) {
            if (isUnauthorized(error)) {
                setUser(null);
                return;
            }

            throw error;
        }
    };

    const isAuthenticated = Boolean(user);

    useEffect(() => {
        void refresh().catch((error) => {
            console.error('Error restoring auth session:', error);
        });
    }, []);

    const value: AuthContextType = {
        user,
        login,
        logout,
        refresh,
        sub: user?.username || null,
        isAuthenticated,
        isLoading,
    };

    return <AuthContext value={value}>{children}</AuthContext>;
};
