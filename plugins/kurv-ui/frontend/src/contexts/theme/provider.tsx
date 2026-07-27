import { type ReactNode, useEffect, useState } from 'react';
import { ThemeContext, type ThemeMode } from './context';

const THEME_STORAGE_KEY = 'kurv_theme_mode';

const isThemeMode = (value: string | null): value is ThemeMode => {
    return value === 'light' || value === 'dark';
};

const getSystemTheme = (): ThemeMode => {
    if (
        typeof window !== 'undefined' &&
        window.matchMedia('(prefers-color-scheme: dark)').matches
    ) {
        return 'dark';
    }

    return 'light';
};

const getInitialTheme = (): ThemeMode => {
    if (typeof window === 'undefined') {
        return 'light';
    }

    const storedTheme = localStorage.getItem(THEME_STORAGE_KEY);

    if (isThemeMode(storedTheme)) {
        return storedTheme;
    }

    return getSystemTheme();
};

export const ThemeProvider = ({ children }: { children: ReactNode }) => {
    const [mode, setMode] = useState<ThemeMode>(getInitialTheme);

    useEffect(() => {
        document.documentElement.dataset.theme = mode;
        document.documentElement.style.colorScheme = mode;
        localStorage.setItem(THEME_STORAGE_KEY, mode);
    }, [mode]);

    useEffect(() => {
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

        const handleStorageChange = (event: StorageEvent) => {
            if (event.key && event.key !== THEME_STORAGE_KEY) {
                return;
            }

            if (isThemeMode(event.newValue)) {
                setMode(event.newValue);
                return;
            }

            setMode(getSystemTheme());
        };

        const handleSystemThemeChange = (event: MediaQueryListEvent) => {
            const storedTheme = localStorage.getItem(THEME_STORAGE_KEY);

            if (!isThemeMode(storedTheme)) {
                setMode(event.matches ? 'dark' : 'light');
            }
        };

        window.addEventListener('storage', handleStorageChange);
        mediaQuery.addEventListener('change', handleSystemThemeChange);

        return () => {
            window.removeEventListener('storage', handleStorageChange);
            mediaQuery.removeEventListener('change', handleSystemThemeChange);
        };
    }, []);

    return (
        <ThemeContext
            value={{
                mode,
                setMode,
            }}
        >
            {children}
        </ThemeContext>
    );
};
