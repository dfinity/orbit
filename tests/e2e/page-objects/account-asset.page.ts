import { expect, Page } from '@playwright/test';

export class AccountAssetPage {
  constructor(private page: Page) {}

  balance() {
    return this.page.getByTestId('page-header-title');
  }

  async getIcpNativeAddress() {
    return await this.page.getByTestId('icp_account_identifier').getAttribute('title');
  }

  async getBalance() {
    return await this.balance().textContent();
  }

  /**
   * Waits until the balance shown in the page header contains the expected value.
   *
   * The page refreshes its own balance every five seconds and shows a placeholder until the first
   * one arrives, so this waits on the live header. Between attempts the asset page is opened again
   * (the caller has already waited for the asset route, so this cannot race the navigation), which
   * unblocks a page left in an unexpected state such as a failed load.
   */
  async expectBalance(balance: string | RegExp, timeout = 180_000) {
    const assetPageUrl = this.page.url();
    let attempt = 0;

    await expect(async () => {
      if (attempt++ > 0) {
        await this.page.goto(assetPageUrl);
      }

      await expect(this.balance()).toContainText(balance, { timeout: 30_000 });
    }).toPass({ timeout });
  }
}
