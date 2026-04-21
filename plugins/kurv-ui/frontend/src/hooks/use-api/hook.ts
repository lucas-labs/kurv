import { aget, apost, get, post } from 'kypi';
import { createClientHook } from 'kypi/react';
import { use, useCallback } from 'react';
import { Globals } from '@/contexts/globals/context';
import { useAuth } from '../use-auth';
import type {
    KurvEgg,
    KurvEggSummaryList,
    ListEggsQuery,
    LoginRequest,
    LoginResponse,
    SetupInitializeRequest,
    SetupInitializeResponse,
    SetupStatusResponse,
} from './types';

const useClient = createClientHook({
    auth: {
        login: post<LoginRequest, LoginResponse>('/auth/login'),
    },
    admin: {
        setup: {
            status: get<void, SetupStatusResponse>('/setup/status'),
            initialize: post<SetupInitializeRequest, SetupInitializeResponse>(
                '/setup/initial-user',
            ),
        },
    },
    kurv: {
        eggs: {
            list: aget<void, KurvEggSummaryList, undefined, ListEggsQuery>('/kurv/eggs'),
            get: aget<void, KurvEgg, { eggId: string }>('/kurv/eggs/:eggId'),
            start: apost<void, KurvEgg, { eggId: string }>('/kurv/eggs/:eggId/start'),
            stop: apost<void, KurvEgg, { eggId: string }>('/kurv/eggs/:eggId/stop'),
            restart: apost<void, KurvEgg, { eggId: string }>('/kurv/eggs/:eggId/restart'),
        },
    },
});

type ParlaApiClient = ReturnType<typeof useClient>;

type ParlaApiConfig = {
    baseUrl: string;
};

export function useApi(): ParlaApiClient;
export function useApi<T>(selector: (api: ParlaApiClient) => T): T;
export function useApi<T>(
    selector: (api: ParlaApiClient) => T | undefined,
    config?: ParlaApiConfig,
): ParlaApiClient | T;
export function useApi(config?: ParlaApiConfig): ParlaApiClient;
export function useApi<T>(
    firstArg?: ((api: ParlaApiClient) => T) | ParlaApiConfig,
    secondArg?: ParlaApiConfig,
): ParlaApiClient | T {
    const { token } = useAuth();
    const { server } = use(Globals);
    const selector = typeof firstArg === 'function' ? firstArg : undefined;
    const config = typeof firstArg === 'object' ? firstArg : secondArg;
    const getToken = useCallback(() => token, [token]);

    const api = useClient({
        baseUrl: config?.baseUrl || `http://${server.baseUrl}`,
        getToken,
    });

    if (selector) {
        return selector(api);
    }

    return api;
}
