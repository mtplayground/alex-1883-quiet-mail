import { Button } from '../components/ui/Button';

type LoginPageProps = {
  error: string | null;
  onLogin: () => void;
};

export function LoginPage({ error, onLogin }: LoginPageProps) {
  return (
    <main className="grid min-h-screen place-items-center bg-canvas px-6 py-12 text-ink">
      <section className="w-full max-w-sm">
        <p className="text-xs font-semibold uppercase text-ink-muted">Secure access</p>
        <h1 className="mt-3 text-3xl font-semibold text-ink">A quieter mailbox</h1>
        <p className="mt-4 text-sm leading-6 text-ink-muted">
          Sign in to continue to your mail workspace.
        </p>

        {error ? (
          <p className="mt-6 rounded-ui border border-line bg-surface px-3 py-2 text-sm text-ink-muted">
            {error}
          </p>
        ) : null}

        <Button className="mt-8 justify-center" onClick={onLogin} variant="primary">
          Sign in
        </Button>
      </section>
    </main>
  );
}
