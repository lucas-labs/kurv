import { createContext } from 'react';

export interface AuthContextType {
    token: string | null;
    login: (token: string) => void;
    logout: () => void;
    sub: string | null;
    isAuthenticated: boolean;
}

/** Context for authentication state and actions */
export const AuthContext = createContext<AuthContextType | undefined>(undefined);
