import ky from 'ky';
import { aget, apost, get, post } from 'kypi';
import { createClientHook } from 'kypi/react';
import { use } from 'react';
import { Globals } from '@/contexts/globals/context';
import type {
    CurrentUserResponse,
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
        logout: post<void, void>('/auth/logout'),
        me: get<void, CurrentUserResponse>('/auth/me'),
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

const cookieAwareKy = ky.create({
    credentials: 'include',
});

type ParlaApiClient = ReturnType<typeof useClient>;

type ParlaApiConfig = {
    baseUrl: string;
};

const resolveBaseUrl = (baseUrl: string) => {
    if (baseUrl.startsWith('http://') || baseUrl.startsWith('https://')) {
        return baseUrl;
    }

    if (baseUrl.startsWith('/')) {
        return baseUrl;
    }

    return `http://${baseUrl}`;
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
    const { server } = use(Globals);
    const selector = typeof firstArg === 'function' ? firstArg : undefined;
    const config = typeof firstArg === 'object' ? firstArg : secondArg;

    const api = useClient({
        baseUrl: resolveBaseUrl(config?.baseUrl || server.baseUrl),
        kyInstance: cookieAwareKy,
    });

    if (selector) {
        return selector(api);
    }

    return api;
}
