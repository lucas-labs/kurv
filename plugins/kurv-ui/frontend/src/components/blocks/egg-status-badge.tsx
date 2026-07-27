import type { ComponentProps } from 'react';
import type { KurvEggStatus } from '@/hooks/use-api/types';
import { cn } from '@/lib/utils';

const DOT_COLORS: Record<KurvEggStatus, string> = {
    Pending: 'bg-amber-400',
    Running: 'bg-emerald-400',
    Stopped: 'bg-red-500',
    PendingRemoval: 'bg-orange-400',
    Restarting: 'bg-sky-400',
    Errored: 'bg-red-400',
};

type EggStatusBadgeProps = ComponentProps<'span'> & {
    status: KurvEggStatus;
};

export function EggStatusBadge({ status, className, ...props }: EggStatusBadgeProps) {
    return (
        <span
            className={cn(
                'text-muted-foreground inline-flex items-center gap-1.5 font-mono text-xs',
                className,
            )}
            {...props}
        >
            <span className={cn('inline-block size-1.5 rounded-full', DOT_COLORS[status])} />
            {status.toLowerCase()}
        </span>
    );
}
