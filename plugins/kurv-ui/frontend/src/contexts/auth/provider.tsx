import { type ReactNode, useEffect, useState } from 'react';
import { AuthContext, type AuthContextType } from './context';

const TOKEN_STORAGE_KEY = 'parla_auth_token';

export const AuthProvider = ({ children }: { children: ReactNode }) => {
    const [token, setToken] = useState<string | null>(() => {
        // Initialize from localStorage if available
        if (typeof window !== 'undefined') {
            return localStorage.getItem(TOKEN_STORAGE_KEY);
        }
        return null;
    });

    const login = (newToken: string) => {
        setToken(newToken);
        localStorage.setItem(TOKEN_STORAGE_KEY, newToken);
    };

    const logout = () => {
        setToken(null);
        localStorage.removeItem(TOKEN_STORAGE_KEY);
    };

    const isAuthenticated = Boolean(token);

    useEffect(() => {
        // Sync with localStorage changes from other tabs
        const handleStorageChange = (e: StorageEvent) => {
            if (e.key === TOKEN_STORAGE_KEY) {
                setToken(e.newValue);
            }
        };

        window.addEventListener('storage', handleStorageChange);
        return () => window.removeEventListener('storage', handleStorageChange);
    }, []);

    const value: AuthContextType = {
        token,
        login,
        logout,
        sub: token ? JSON.parse(atob(token.split('.')[1])).sub : null,
        isAuthenticated,
    };

    return <AuthContext value={value}>{children}</AuthContext>;
};
