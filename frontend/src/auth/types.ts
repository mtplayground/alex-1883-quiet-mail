export type AuthUser = {
  sub: string;
  email: string;
  name: string | null;
  picture_url: string | null;
  registered: boolean;
};

export type AuthSession = {
  authenticated: true;
  user: AuthUser;
};
