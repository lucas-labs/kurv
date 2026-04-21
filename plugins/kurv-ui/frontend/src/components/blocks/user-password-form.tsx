import { type ComponentProps, useState } from 'react';
import { Button } from '../ui/button';
import { LabeledInput } from './labeled-input';

export type UserPasswordFormProps = Omit<ComponentProps<'form'>, 'onSubmit'> & {
    onSubmit?: (username: string, password: string) => void | Promise<void>;
    isLoading?: boolean;
    title?: string;
    description?: string;
    buttonText?: string;
    buttonLoadingText?: string;
};

const UserPasswordForm = ({
    className,
    title,
    description,
    isLoading = false,
    buttonText = 'Ingresar',
    buttonLoadingText = 'Ingresando...',
    onSubmit,
    ...props
}: UserPasswordFormProps) => {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        const trimmedUsername = username.trim();
        const trimmedPassword = password.trim();

        onSubmit?.(trimmedUsername, trimmedPassword);
    };

    return (
        <form onSubmit={handleSubmit} className={className} {...props}>
            <div className="flex flex-col gap-5">
                {(title || description) && (
                    <div className="flex flex-col items-center gap-1 text-center">
                        {title && <h1 className="not-prose text-2xl font-bold">{title}</h1>}
                        {description && (
                            <p className="not-prose text-muted-foreground text-pretty">
                                {description}
                            </p>
                        )}
                    </div>
                )}
                <LabeledInput
                    label="Usuario"
                    id="username"
                    type="text"
                    placeholder="mi-usuario"
                    autoComplete="username"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                    required
                />

                <LabeledInput
                    label="Contraseña"
                    id="password"
                    type="password"
                    value={password}
                    autoComplete="current-password"
                    onChange={(e) => setPassword(e.target.value)}
                    placeholder="contraseña"
                    required
                />

                <Button
                    type="submit"
                    variant="outline"
                    className="mt-2 w-full"
                    disabled={isLoading}
                >
                    {isLoading ? buttonLoadingText : buttonText}
                </Button>
            </div>
        </form>
    );
};

export { UserPasswordForm };
