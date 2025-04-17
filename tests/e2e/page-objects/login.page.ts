import { expect, Page } from '@playwright/test';
import { InternetIdentityPage } from './internet-identity.page';

export class LoginPage {
  constructor(private page: Page) {}

  async login() {
    await this.page.getByTestId('internet-identity-button').click();
  }

  async register(): Promise<string> {
    const internetIdentityPagePromise = this.page.waitForEvent('popup');
    await this.page.getByTestId('internet-identity-button').click();

    const ii = new InternetIdentityPage(await internetIdentityPagePromise);
    return await ii.register();
  }

  async assertOnLoginPage() {
    await expect(this.page).toHaveURL(/login/);
  }
}
