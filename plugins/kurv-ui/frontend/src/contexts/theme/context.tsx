import { createContext } from 'react';

export type ThemeMode = 'light' | 'dark';

export interface ThemeType {
    mode?: ThemeMode;
    setMode?: (mode: ThemeMode) => void;
}

/** Context for managing themes */
export const ThemeContext = createContext<ThemeType | undefined>(undefined);
