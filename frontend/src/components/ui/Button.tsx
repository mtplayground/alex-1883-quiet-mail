import type { ButtonHTMLAttributes } from 'react';

import { classNames } from '../../lib/classNames';

type ButtonVariant = 'primary' | 'quiet';

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: ButtonVariant;
};

const variants: Record<ButtonVariant, string> = {
  primary: 'bg-accent text-white hover:bg-accent-strong',
  quiet: 'bg-transparent text-ink-muted hover:bg-accent-soft hover:text-ink',
};

export function Button({ className, type = 'button', variant = 'quiet', ...props }: ButtonProps) {
  return (
    <button
      className={classNames(
        'inline-flex min-h-10 w-full items-center justify-start rounded-ui px-3 text-sm font-medium transition-colors focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent',
        variants[variant],
        className,
      )}
      type={type}
      {...props}
    />
  );
}
