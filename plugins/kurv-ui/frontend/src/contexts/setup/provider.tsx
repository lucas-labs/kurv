import { type ReactNode, useEffect, useEffectEvent, useState } from 'react';
import { useApi } from '@/hooks/use-api/hook';
import type { SetupStatus } from '@/hooks/use-api/types';
import { SetupStatusContext } from './context';

export const SetupStatusProvider = ({ children }: { children: ReactNode }) => {
    const api = useApi((api) => api.admin.setup);
    const [status, setStatus] = useState<SetupStatus>('loading');
    const [error, setError] = useState<string | undefined>(undefined);

    const fetchStatus = useEffectEvent(async () => {
        try {
            const response = await api.status().json();
            setStatus(response.status);
            setError(undefined);
        } catch (error) {
            console.error('Error fetching setup status:', error);
            setStatus('errored');
            setError(
                error instanceof Error
                    ? error.message
                    : error instanceof Response
                      ? await error.text()
                      : typeof error === 'string'
                        ? error
                        : 'Unknown error',
            );
        }
    });

    useEffect(() => {
        void fetchStatus();
    }, []);

    return (
        <SetupStatusContext value={{ status, fetchStatus, setStatus, error }}>
            {children}
        </SetupStatusContext>
    );
};
