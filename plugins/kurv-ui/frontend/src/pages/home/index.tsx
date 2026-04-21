import { Pause, RefreshCw } from 'lucide-react';
import { Link } from 'react-router';
import { toast } from 'sonner';
import {
    startTransition,
    useCallback,
    useEffect,
    useEffectEvent,
    useMemo,
    useRef,
    useState,
} from 'react';
import { type EggAction, EggActionButtons } from '@/components/blocks/egg-action-buttons';
import { EggStatusBadge } from '@/components/blocks/egg-status-badge';
import { IconButton } from '@/components/blocks/icon-button';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { useApi } from '@/hooks/use-api/hook';
import type { KurvEggKind, KurvEggStatus, KurvEggSummary } from '@/hooks/use-api/types';
import { cn } from '@/lib/utils';

const FILTERS: Array<{ value: KurvEggKind; label: string }> = [
    { value: 'eggs', label: 'Eggs' },
    { value: 'plugins', label: 'Plugins' },
];

type PendingActionState = {
    eggId: string;
    action: EggAction;
} | null;

export function HomePage() {
    const api = useApi((client) => client.kurv.eggs);
    const [kind, setKind] = useState<KurvEggKind>('eggs');
    const [eggs, setEggs] = useState<KurvEggSummary[]>([]);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [pendingAction, setPendingAction] = useState<PendingActionState>(null);
    const [autoRefresh, setAutoRefresh] = useState(true);
    const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

    const clearAutoRefresh = useCallback(() => {
        if (intervalRef.current) {
            clearInterval(intervalRef.current);
            intervalRef.current = null;
        }
    }, []);

    const stats = useMemo(() => {
        const countByStatus = eggs.reduce<Record<KurvEggStatus, number>>(
            (acc, egg) => {
                acc[egg.status] += 1;
                return acc;
            },
            {
                Pending: 0,
                Running: 0,
                Stopped: 0,
                PendingRemoval: 0,
                Restarting: 0,
                Errored: 0,
            },
        );

        return {
            total: eggs.length,
            running: countByStatus.Running,
            inactive: countByStatus.Stopped + countByStatus.Pending + countByStatus.PendingRemoval,
            issues: countByStatus.Errored,
        };
    }, [eggs]);

    const fetchEggs = useEffectEvent(async (nextKind: KurvEggKind, silent = false) => {
        if (!silent) {
            setIsLoading(true);
        }

        try {
            const response = await api.list({ query: { kind: nextKind } }).json();

            startTransition(() => {
                setEggs(response);
                setError(null);
            });
        } catch (err) {
            console.error('Failed to fetch eggs:', err);
            if (!silent) {
                setError(err instanceof Error ? err.message : 'Failed to fetch eggs.');
            }
        } finally {
            if (!silent) {
                setIsLoading(false);
            }
        }
    });

    const handleAction = useEffectEvent(async (eggId: string, action: EggAction) => {
        setPendingAction({ eggId, action });

        try {
            await api[action]({ params: { eggId } }).json();
            toast.success(`Egg ${action} request sent.`);
            await fetchEggs(kind);
        } catch (err) {
            console.error(`Failed to ${action} egg:`, err);
            toast.error(err instanceof Error ? err.message : `Failed to ${action} egg.`);
        } finally {
            setPendingAction(null);
        }
    });

    useEffect(() => {
        void fetchEggs(kind);
    }, [kind]);

    useEffect(() => {
        clearAutoRefresh();

        if (autoRefresh) {
            intervalRef.current = setInterval(() => {
                void fetchEggs(kind, true);
            }, 10_000);
        }

        return clearAutoRefresh;
    }, [autoRefresh, kind, clearAutoRefresh]);

    return (
        <div className="flex flex-col gap-10">
            {/* Page header */}
            <div className="flex flex-col gap-6">
                <div className="flex items-end justify-between">
                    <div>
                        <h1 className="text-3xl md:text-4xl">Processes</h1>
                        <p className="text-muted-foreground mt-1 text-sm">
                            Managed processes and plugins.
                        </p>
                    </div>

                    <div className="flex items-center gap-2">
                        <Tooltip>
                            <TooltipTrigger
                                render={
                                    <button
                                        onClick={() => setAutoRefresh((prev) => !prev)}
                                        className={cn(
                                            'text-muted-foreground hover:text-foreground rounded-full p-1 text-xs transition-colors',
                                            autoRefresh && 'text-emerald-400',
                                        )}
                                    />
                                }
                            >
                                {autoRefresh ? (
                                    <Pause className="size-3" />
                                ) : (
                                    <RefreshCw className="size-3" />
                                )}
                            </TooltipTrigger>
                            <TooltipContent>
                                {autoRefresh ? 'Pause auto-refresh' : 'Enable auto-refresh'}
                            </TooltipContent>
                        </Tooltip>

                        <IconButton
                            icon={RefreshCw}
                            tooltip="Refresh now"
                            onClick={() => void fetchEggs(kind)}
                            iconClassName={cn('size-4', isLoading && 'animate-spin')}
                        />
                    </div>
                </div>

                {/* Inline stats + filters */}
                <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
                    <div className="flex items-center gap-1">
                        {FILTERS.map(({ value, label }) => (
                            <button
                                key={value}
                                onClick={() => startTransition(() => setKind(value))}
                                className={cn(
                                    'rounded-full px-3 py-1 text-sm transition-colors',
                                    kind === value
                                        ? 'bg-foreground text-background'
                                        : 'text-muted-foreground hover:text-foreground',
                                )}
                            >
                                {label}
                            </button>
                        ))}
                    </div>

                    <div className="text-muted-foreground flex items-center gap-4 font-mono text-xs">
                        <span>{stats.total} total</span>
                        <span className="text-emerald-400">{stats.running} running</span>
                        {stats.inactive > 0 && <span>{stats.inactive} inactive</span>}
                        {stats.issues > 0 && (
                            <span className="text-red-400">{stats.issues} errored</span>
                        )}
                    </div>
                </div>
            </div>

            {/* Process list */}
            <div className="border-t">
                {isLoading ? (
                    <ListMessage label="Loading..." />
                ) : error ? (
                    <ListMessage label={error} tone="error" />
                ) : eggs.length === 0 ? (
                    <ListMessage label={`No ${kind} found.`} />
                ) : (
                    eggs.map((egg) => (
                        <article
                            key={egg.id}
                            className="group grid gap-4 border-b py-5 md:grid-cols-[minmax(0,1fr)_auto]"
                        >
                            <div className="space-y-2">
                                <div className="flex flex-wrap items-center gap-3">
                                    <Link
                                        to={`/eggs/${egg.id}`}
                                        className="hover:text-muted-foreground text-lg font-semibold transition-colors"
                                    >
                                        {egg.name}
                                    </Link>
                                    <EggStatusBadge status={egg.status} />
                                </div>

                                <div className="text-muted-foreground flex flex-wrap gap-x-4 font-mono text-xs">
                                    <span>id:{egg.id}</span>
                                    <span>pid:{formatPid(egg.pid)}</span>
                                    {egg.uptime && <span>up:{egg.uptime}</span>}
                                    <span>retries:{egg.retry_count}</span>
                                </div>
                            </div>

                            <div className="flex items-center gap-3">
                                <EggActionButtons
                                    status={egg.status}
                                    pendingAction={
                                        pendingAction?.eggId === String(egg.id)
                                            ? pendingAction.action
                                            : null
                                    }
                                    onAction={(action) => void handleAction(String(egg.id), action)}
                                />

                                <Link
                                    to={`/eggs/${egg.id}`}
                                    className="text-muted-foreground hover:text-foreground text-sm transition-colors"
                                >
                                    →
                                </Link>
                            </div>
                        </article>
                    ))
                )}
            </div>
        </div>
    );
}

function ListMessage({ label, tone = 'default' }: { label: string; tone?: 'default' | 'error' }) {
    return (
        <div
            className={cn(
                'text-muted-foreground flex min-h-40 items-center justify-center text-sm',
                tone === 'error' && 'text-destructive',
            )}
        >
            {label}
        </div>
    );
}

function formatPid(pid: number) {
    return pid > 0 ? pid : '-';
}
