export interface Page<T> {
    data: T[];
    total: number;
    page: number;
    pageSize: number;
}

export interface PaginationParams {
    page?: number;
    pageSize?: number;
}

// Authentication types

export interface LoginRequest {
    username: string;
    password: string;
}

export interface AuthenticatedUser {
    username: string;
}

export type LoginResponse = AuthenticatedUser;
export type CurrentUserResponse = AuthenticatedUser;

// Admin: Setup API types

export type SetupStatus = 'uninitialized' | 'ready' | 'loading' | 'errored';
export type SetupStatusResponse = {
    status: SetupStatus;
};
export interface SetupInitializeRequest {
    username: string;
    password: string;
}
export type SetupInitializeResponse = Pick<SetupInitializeRequest, 'username'>;

// Kurv eggs API types

export type KurvEggKind = 'eggs' | 'plugins';

export type KurvEggStatus =
    | 'Pending'
    | 'Running'
    | 'Stopped'
    | 'PendingRemoval'
    | 'Restarting'
    | 'Errored';

export interface KurvEggSummary {
    id: number;
    pid: number;
    name: string;
    status: KurvEggStatus;
    uptime: string;
    retry_count: number;
}

export type KurvEggSummaryList = KurvEggSummary[];

export interface KurvEggState {
    status: KurvEggStatus;
    start_time: string | null;
    try_count: number;
    error: string | null;
    pid: number;
}

export interface KurvEggPaths {
    stdout: string;
    stderr: string;
}

export interface KurvEgg {
    command: string;
    name: string;
    id: number | null;
    state: KurvEggState | null;
    args: string[] | null;
    cwd: string | null;
    env: Record<string, string> | null;
    paths: KurvEggPaths | null;
    plugin: boolean | null;
    plugin_path: string | null;
}

export interface ListEggsQuery {
    kind?: KurvEggKind;
}
