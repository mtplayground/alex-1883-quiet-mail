import type { AuthUser } from '../../auth/types';

type UserBadgeProps = {
  user: AuthUser;
};

export function UserBadge({ user }: UserBadgeProps) {
  const displayName = user.name ?? user.email;

  return (
    <div className="flex min-w-0 items-center gap-3">
      {user.picture_url ? (
        <img
          alt=""
          className="size-10 rounded-full border border-line object-cover"
          referrerPolicy="no-referrer"
          src={user.picture_url}
        />
      ) : (
        <div className="grid size-10 place-items-center rounded-full border border-line bg-accent-soft text-sm font-semibold text-accent-strong">
          {displayName.slice(0, 1).toUpperCase()}
        </div>
      )}
      <div className="min-w-0 text-right">
        <p className="truncate text-sm font-medium text-ink">{displayName}</p>
        <p className="truncate text-xs text-ink-muted">{user.email}</p>
      </div>
    </div>
  );
}
