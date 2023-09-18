export enum AuthState {
  Authenticated = 'authenticated',
  Guest = 'guest',
  Any = 'any',
}

export interface AuthRouteMeta {
  requireState: AuthState;
}

declare module 'vue-router' {
  interface RouteMeta {
    // must be declared by every route
    auth: AuthRouteMeta
  }
}
