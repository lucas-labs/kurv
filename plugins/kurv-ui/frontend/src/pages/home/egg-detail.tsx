import { HTTPError } from 'ky';
import { ArrowLeft, RefreshCw } from 'lucide-react';
import { Link, Navigate, useParams } from 'react-router';
import { toast } from 'sonner';
import {
    type ReactNode,
    startTransition,
    useEffect,
    useEffectEvent,
    useMemo,
    useState,
} from 'react';
import { type EggAction, EggActionButtons } from '@/components/blocks/egg-action-buttons';
import { EggStatusBadge } from '@/components/blocks/egg-status-badge';
import { IconButton } from '@/components/blocks/icon-button';
import { useApi } from '@/hooks/use-api/hook';
import type { KurvEgg } from '@/hooks/use-api/types';
import { cn } from '@/lib/utils';

export function EggDetailPage() {
    const { eggId } = useParams();
    const api = useApi((client) => client.kurv.eggs);
    const [egg, setEgg] = useState<KurvEgg | null>(null);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [pendingAction, setPendingAction] = useState<EggAction | null>(null);

    const orderedEnv = useMemo(
        () => Object.entries(egg?.env ?? {}).sort(([left], [right]) => left.localeCompare(right)),
        [egg?.env],
    );

    const fetchEgg = useEffectEvent(async () => {
        if (!eggId) {
            return;
        }

        setIsLoading(true);

        try {
            const response = await api.get({ params: { eggId } }).json();

            startTransition(() => {
                setEgg(response);
                setError(null);
            });
        } catch (err) {
            console.error('Failed to fetch egg detail:', err);
            setError(getKurvRequestErrorMessage(err, 'Failed to fetch egg detail.'));
        } finally {
            setIsLoading(false);
        }
    });

    const handleAction = useEffectEvent(async (action: EggAction) => {
        if (!eggId) {
            return;
        }

        setPendingAction(action);

        try {
            const response = await api[action]({ params: { eggId } }).json();
            setEgg(response);
            toast.success(`Egg ${action} request sent.`);
        } catch (err) {
            console.error(`Failed to ${action} egg:`, err);
            toast.error(getKurvRequestErrorMessage(err, `Failed to ${action} egg.`));
        } finally {
            setPendingAction(null);
        }
    });

    useEffect(() => {
        void fetchEgg();
    }, [eggId]);

    if (!eggId) {
        return <Navigate to="/eggs" replace />;
    }

    if (isLoading) {
        return <DetailMessage label="Loading..." />;
    }

    if (error || !egg) {
        return <DetailMessage label={error || 'Egg not found.'} tone="error" />;
    }

    return (
        <div className="flex flex-col gap-10">
            {/* Header */}
            <div className="flex flex-col gap-5">
                <Link
                    to="/eggs"
                    className="text-muted-foreground hover:text-foreground inline-flex items-center gap-1.5 text-sm transition-colors"
                >
                    <ArrowLeft className="size-3" />
                    Back
                </Link>

                <div className="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
                    <div className="space-y-3">
                        <div className="flex flex-wrap items-center gap-3">
                            <h1 className="text-3xl md:text-4xl">{egg.name}</h1>
                            {egg.state?.status && <EggStatusBadge status={egg.state.status} />}
                        </div>

                        <div className="text-muted-foreground flex flex-wrap gap-x-4 font-mono text-xs">
                            <span>{egg.plugin ? 'plugin' : 'egg'}</span>
                            <span>id:{egg.id ?? '-'}</span>
                            <span>pid:{egg.state?.pid || '-'}</span>
                            <span>attempts:{egg.state?.try_count ?? 0}</span>
                        </div>
                    </div>

                    <div className="flex items-center gap-3">
                        <EggActionButtons
                            status={egg.state?.status}
                            pendingAction={pendingAction}
                            onAction={(action) => void handleAction(action)}
                        />
                        <IconButton
                            icon={RefreshCw}
                            tooltip="Refresh"
                            onClick={() => void fetchEgg()}
                            iconClassName="size-4"
                        />
                    </div>
                </div>
            </div>

            {/* Sections */}
            <div className="grid gap-10 xl:grid-cols-2">
                <Section title="Command">
                    <KV label="command" value={egg.command} mono />
                    <KV label="cwd" value={egg.cwd || '-'} mono />
                    <KV label="args" value={egg.args?.length ? egg.args.join(' ') : '-'} mono />
                    {egg.plugin_path && <KV label="plugin" value={egg.plugin_path} mono />}
                </Section>

                <Section title="Runtime">
                    <KV label="status" value={egg.state?.status || '-'} />
                    <KV label="started" value={formatDate(egg.state?.start_time)} />
                    <KV label="pid" value={String(egg.state?.pid || '-')} mono />
                    {egg.state?.error && <KV label="error" value={egg.state.error} />}
                </Section>

                <Section title="Log paths">
                    <KV label="stdout" value={egg.paths?.stdout || '-'} mono />
                    <KV label="stderr" value={egg.paths?.stderr || '-'} mono />
                </Section>

                <Section title="Environment">
                    {orderedEnv.length === 0 ? (
                        <p className="text-muted-foreground text-sm">No entries.</p>
                    ) : (
                        <div className="max-h-80 overflow-auto">
                            {orderedEnv.map(([key, value]) => (
                                <div key={key} className="flex gap-4 border-b py-2 last:border-0">
                                    <span className="text-muted-foreground shrink-0 font-mono text-xs">
                                        {key}
                                    </span>
                                    <code className="text-xs break-all">{value}</code>
                                </div>
                            ))}
                        </div>
                    )}
                </Section>
            </div>
        </div>
    );
}

function getKurvRequestErrorMessage(error: unknown, fallback: string) {
    if (error instanceof HTTPError && error.response.status === 502) {
        return 'Unable to reach the kurv server. Make sure it is running and try again.';
    }

    return error instanceof Error ? error.message : fallback;
}

function Section({ title, children }: { title: string; children: ReactNode }) {
    return (
        <section>
            <h2 className="mb-4 font-sans text-sm font-semibold tracking-wide">{title}</h2>
            <div className="space-y-0 divide-y border-l-2 border-white/10 pl-5">{children}</div>
        </section>
    );
}

function KV({ label, value, mono = false }: { label: string; value: string; mono?: boolean }) {
    return (
        <div className="flex flex-col gap-0.5 py-3 sm:flex-row sm:items-baseline sm:gap-6">
            <span className="w-24 shrink-0 text-sm">{label}</span>
            <span className={cn('text-muted-foreground text-sm break-all', mono && 'font-mono')}>
                {value}
            </span>
        </div>
    );
}

function DetailMessage({ label, tone = 'default' }: { label: string; tone?: 'default' | 'error' }) {
    return (
        <div className="flex min-h-40 items-center justify-center">
            <p
                className={cn(
                    'text-muted-foreground text-sm',
                    tone === 'error' && 'text-destructive',
                )}
            >
                {label}
            </p>
        </div>
    );
}

function formatDate(value?: string | null) {
    if (!value) {
        return '-';
    }

    const date = new Date(value);
    if (Number.isNaN(date.getTime())) {
        return value;
    }

    return date.toLocaleString();
}
