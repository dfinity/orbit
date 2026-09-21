import { expect, Page } from '@playwright/test';

export class AccountAssetPage {
  constructor(private page: Page) {}

  async getIcpNativeAddress() {
    return await this.page.getByTestId('icp_account_identifier').getAttribute('title');
  }

  async getBalance() {
    return await this.page.getByTestId('page-header-title').textContent();
  }

  // The page refreshes its own balance every five seconds and shows a placeholder until the first
  // one arrives, so this waits on the live header. Reloading instead would race a navigation that
  // has not settled yet and read the account page header, which is the account name.
  async expectBalance(balance: string, timeout = 180_000) {
    await expect(this.page.getByTestId('page-header-title')).toContainText(balance, { timeout });
  }
}
