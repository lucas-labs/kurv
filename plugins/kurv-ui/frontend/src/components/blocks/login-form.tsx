import { type ComponentProps } from 'react';
import { cn } from '@/lib/utils';
import { UserPasswordForm } from './user-password-form';

export type LoginFormProps = Omit<ComponentProps<'div'>, 'onSubmit'> & {
    onSubmit?: (username: string, password: string) => void | Promise<void>;
    isLoading?: boolean;
    errorMessage?: string | null;
    imageId?: number;
};

export function LoginForm({
    className,
    imageId,
    onSubmit,
    isLoading,
    errorMessage,
    ...props
}: LoginFormProps) {
    return (
        <div className={cn('relative flex flex-col gap-6', className)} {...props}>
            <UserPasswordForm
                onSubmit={onSubmit}
                isLoading={isLoading}
                errorMessage={errorMessage}
            />
        </div>
    );
}
