import { isApiError } from './app.utils';

export function getErrorMessage(error: unknown): string {
  if (isApiError(error) && error.message.length > 0) {
    return error.message[0]!;
  } else if (error instanceof Error) {
    return error.message;
  }
  return String(error);
}
