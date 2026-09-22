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
   * The page refreshes the balance on its own, but between attempts the asset page is opened again
   * so that a page left in an unexpected state (e.g. a failed load) cannot block the wait.
   */
  async waitForBalance(expected: string | RegExp, timeout = 180_000) {
    const assetPageUrl = this.page.url();
    let attempt = 0;

    await expect(async () => {
      if (attempt++ > 0) {
        await this.page.goto(assetPageUrl);
      }

      await expect(this.balance()).toContainText(expected, { timeout: 30_000 });
    }).toPass({ timeout });
  }
}
