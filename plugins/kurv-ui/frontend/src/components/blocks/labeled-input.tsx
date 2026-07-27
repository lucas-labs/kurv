import { type ComponentProps } from 'react';
import { Field, FieldDescription, FieldLabel } from '@/components/ui/field';
import { Input } from '@/components/ui/input';
import { Textarea } from '../ui/textarea';

type Props = {
    id: string;
    label: string;
    description?: string;
    multiline?: boolean;
    inputClassName?: string;
};

type LabeledTextareaProps = Props & { multiline?: true } & ComponentProps<'textarea'>;
type LabeledTextInputProps = Props & { multiline: false } & ComponentProps<'input'>;
export type LabeledInputProps = LabeledTextInputProps | LabeledTextareaProps;

export function LabeledInput({
    id,
    label,
    description,
    multiline = false,
    inputClassName,
    ...props
}: LabeledInputProps) {
    return (
        <Field>
            <FieldLabel htmlFor={id}>{label}</FieldLabel>
            {description && <FieldDescription>{description}</FieldDescription>}
            {multiline === true ? (
                <Textarea
                    id={id}
                    className={inputClassName}
                    {...(props as Omit<LabeledTextareaProps, 'id'>)}
                />
            ) : (
                <Input
                    id={id}
                    className={inputClassName}
                    {...(props as Omit<LabeledTextInputProps, 'id'>)}
                />
            )}
        </Field>
    );
}
