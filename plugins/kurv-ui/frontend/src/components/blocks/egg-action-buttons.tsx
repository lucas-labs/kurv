import { Play, Repeat, Square } from 'lucide-react';
import type { KurvEggStatus } from '@/hooks/use-api/types';
import { cn } from '@/lib/utils';
import { IconButton } from './icon-button';

export type EggAction = 'start' | 'stop' | 'restart';

type EggActionButtonsProps = {
    status?: KurvEggStatus | null;
    pendingAction?: EggAction | null;
    onAction: (action: EggAction) => void | Promise<void>;
    className?: string;
};

export function EggActionButtons({
    status,
    pendingAction,
    onAction,
    className,
}: EggActionButtonsProps) {
    const isBusy = pendingAction !== null;
    const canStart = status !== 'Running' && status !== 'Restarting' && status !== 'PendingRemoval';
    const canStop = status !== 'Stopped' && status !== 'PendingRemoval';
    const canRestart = status !== 'PendingRemoval';

    return (
        <div className={cn('flex items-center gap-1', className)}>
            <IconButton
                icon={Play}
                tooltip="Start"
                disabled={isBusy || !canStart}
                onClick={() => onAction('start')}
            />

            <IconButton
                icon={Square}
                tooltip="Stop"
                disabled={isBusy || !canStop}
                onClick={() => onAction('stop')}
            />

            <IconButton
                icon={Repeat}
                tooltip="Restart"
                disabled={isBusy || !canRestart}
                onClick={() => onAction('restart')}
                iconClassName={pendingAction === 'restart' ? 'animate-spin' : undefined}
            />
        </div>
    );
}
