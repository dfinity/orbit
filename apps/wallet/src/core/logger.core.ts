import pino, { Level, LogEvent } from 'pino';
import { appInitConfig } from '~/configs/init.config';

// Takes the log message, parses it through an annonymizer, and sends it to a log collector
const annonymizedSend = async (_level: Level, _logEvent: LogEvent): Promise<void> => {
  // not yet implemented
};

const logger = pino({
  level: appInitConfig.logLevel,
  timestamp: pino.stdTimeFunctions.isoTime,
  browser: {
    transmit: {
      level: 'warn',
      send: annonymizedSend,
    },
  },
  formatters: {
    level: label => {
      return { level: label.toUpperCase() };
    },
  },
});

export { logger };

export default logger;
