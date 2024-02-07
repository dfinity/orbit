import { Identity } from '@dfinity/agent';
import { AuthService } from '~/services/auth.service';

export const loadIdentity = async (): Promise<Identity | null> => {
  const authService = new AuthService();

  return authService.identity();
};
