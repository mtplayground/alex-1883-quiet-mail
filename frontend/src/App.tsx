import { AuthGate } from './auth/AuthGate';
import { AuthProvider } from './auth/AuthContext';
import { useAuth } from './auth/useAuth';
import { MailboxFrame, MailboxProvider } from './mail/MailboxView';
import { ThemeProvider } from './theme/ThemeContext';

export function App() {
  return (
    <ThemeProvider>
      <AuthProvider>
        <AuthGate>
          <AuthenticatedApp />
        </AuthGate>
      </AuthProvider>
      <IdeavibesWatermark />
    </ThemeProvider>
  );
}

function AuthenticatedApp() {
  const { user } = useAuth();

  if (!user) {
    return null;
  }

  return (
    <MailboxProvider>
      <MailboxFrame user={user} />
    </MailboxProvider>
  );
}

function IdeavibesWatermark() {
  async function handleShare() {
    const payload = {
      title: document.title || 'Ideavibes app',
      text: 'Built with Ideavibes.ai',
      url: window.location.href,
    };

    try {
      if (navigator.share) {
        await navigator.share(payload);
      } else if (navigator.clipboard?.writeText) {
        await navigator.clipboard.writeText(window.location.href);
      }
    } catch {
      // Sharing is best-effort only.
    }
  }

  return (
    <div
      className="fixed bottom-3 left-1/2 z-[2147483647] flex -translate-x-1/2 items-center gap-2 rounded-full border border-slate-400/35 bg-slate-900/90 px-3 py-2 text-xs font-medium leading-none text-slate-50 shadow-[0_10px_30px_rgba(15,23,42,0.25)] backdrop-blur sm:bottom-4 sm:left-auto sm:right-4 sm:translate-x-0"
      id="mctai-watermark"
    >
      <a
        className="text-slate-50 no-underline"
        href="https://ideavibes.ai"
        rel="noopener noreferrer"
        target="_blank"
      >
        Built by Ideavibes.ai
      </a>
      <button
        className="border-0 border-l border-slate-400/35 bg-transparent py-0 pl-2 font-inherit text-sky-300"
        onClick={handleShare}
        type="button"
      >
        Share
      </button>
    </div>
  );
}
