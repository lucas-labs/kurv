import type { LucideIcon } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { cn } from '@/lib/utils';

type IconButtonProps = Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, 'children'> & {
    icon: LucideIcon;
    tooltip: string;
    iconClassName?: string;
};

export function IconButton({
    icon: Icon,
    tooltip,
    iconClassName,
    className,
    ...props
}: IconButtonProps) {
    return (
        <Tooltip>
            <TooltipTrigger
                render={
                    <button
                        className={cn(
                            'text-muted-foreground hover:text-foreground rounded-md p-1.5 transition-colors hover:bg-white/5 disabled:pointer-events-none disabled:opacity-30',
                            className,
                        )}
                        {...props}
                    />
                }
            >
                <Icon className={cn('size-3.5', iconClassName)} />
            </TooltipTrigger>
            <TooltipContent>{tooltip}</TooltipContent>
        </Tooltip>
    );
}
