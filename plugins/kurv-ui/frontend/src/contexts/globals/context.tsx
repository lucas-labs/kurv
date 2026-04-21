import { createContext } from 'react';

export interface GlobalsType {
    server: {
        baseUrl: string;
        version: string;
        allowedMimeTypes?: string[];
    };
}

const globals: GlobalsType = {
    server: {
        baseUrl: window.__SERVER__?.BASE_URL || '',
        version: window.__SERVER__?.VERSION || '',
    },
};

/** Context for authentication state and actions */
export const Globals = createContext<GlobalsType>(globals);
