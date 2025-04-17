import { Page } from '@playwright/test';

export class InternetIdentityPage {
  constructor(private page: Page) {}

  async register(): Promise<string> {
    await this.page.locator('#registerButton').click();
    await this.page.locator('#captchaInput').fill('a');
    await this.page.locator('#captchaInput').press('Enter');

    const anchor = await this.page.getByRole('status', { name: 'usernumber' });

    await this.page.locator('#displayUserContinue').click();

    return (await anchor.textContent()) ?? '';
  }
}
