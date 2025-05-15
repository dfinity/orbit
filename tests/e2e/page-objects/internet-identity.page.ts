import { Page } from '@playwright/test';

export class InternetIdentityPage {
  constructor(private page: Page) {}

  async register(): Promise<string> {
    await this.page.locator('#registerButton').click();
    await this.page.locator('#captchaInput').fill('a');
    await this.page.locator('#captchaInput').press('Enter');

    const anchor = await this.page.getByRole('status', { name: 'usernumber' });

    await this.page.locator('#displayUserContinue').click();

    const anchorText = await anchor.textContent();

    await this.page.waitForEvent('close', { timeout: 15000 });

    return anchorText ?? '';
  }
}
