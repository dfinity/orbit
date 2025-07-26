export const dateWithoutTimezone = (date: Date): Date => {
  return new Date(date.getTime() - date.getTimezoneOffset() * 60000);
};

export const startOfDay = (date: Date): Date => {
  const dt = new Date(dateWithoutTimezone(date).getTime());
  dt.setUTCHours(0, 0, 0, 0);

  return dt;
};

export const endOfDay = (date: Date): Date => {
  const dt = new Date(dateWithoutTimezone(date).getTime());
  dt.setUTCHours(23, 59, 59, 999);

  return dt;
};

interface ConvertDateOptions {
  time?: 'keep' | 'start-of-day' | 'end-of-day';
  tz?: 'keep' | 'local';
}

export function convertDate(date: undefined, options?: ConvertDateOptions): undefined;
export function convertDate(date: Date, options?: ConvertDateOptions): Date;
export function convertDate(date: Date | undefined, options?: ConvertDateOptions): Date | undefined;

export function convertDate(
  date: Date | undefined,
  { time, tz = 'keep' }: ConvertDateOptions = {},
): Date | undefined {
  if (!date) {
    return undefined;
  }

  let dt = date;

  if (tz !== 'keep') {
    dt = new Date(dateWithoutTimezone(date).getTime());
  }

  if (time === 'start-of-day') {
    dt = startOfDay(dt);
  } else if (time === 'end-of-day') {
    dt = endOfDay(dt);
  }

  return tz === 'local' ? new Date(dt.getTime() + dt.getTimezoneOffset() * 60000) : dt;
}

export const parseDate = (raw: string): Date => {
  const date = Date.parse(raw);

  if (isNaN(date)) {
    throw new Error(`Invalid date: ${date}`);
  }

  return new Date(date);
};

export function nanoToJsDate(nanoTimestamp: bigint): Date {
  // Convert BigInt to milliseconds by dividing by 1 million
  const milliTimestamp = nanoTimestamp / BigInt(1000000);

  // Convert to number type as JavaScript's Date constructor expects a number for milliseconds
  const jsDate = new Date(Number(milliTimestamp));
  return jsDate;
}

export function formatLocaleDatetimeString(date: Date | string): string {
  if (typeof date === 'string') {
    date = parseDate(date);
  }

  return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
}
