import { createContext } from 'react';
import type { AuthenticatedUser } from '@/hooks/use-api/types';

export interface AuthContextType {
    user: AuthenticatedUser | null;
    login: (username: string, password: string) => Promise<void>;
    logout: () => Promise<void>;
    refresh: () => Promise<void>;
    sub: string | null;
    isAuthenticated: boolean;
    isLoading: boolean;
}

/** Context for authentication state and actions */
export const AuthContext = createContext<AuthContextType | undefined>(undefined);
