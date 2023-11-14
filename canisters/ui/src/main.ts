import { logger } from '~/core';
import { initializeApp } from '~/ui/App';

initializeApp().catch(err => {
  logger.error(`Failed to initialize app`, { err });
});
