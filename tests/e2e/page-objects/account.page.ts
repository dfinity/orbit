import { Page } from '@playwright/test';

export class AccountPage {
  constructor(private page: Page) {}

  async pickByAsset(asset: string) {
    await this.page.getByTestId('account-assets-table').getByText(asset).click();

    // The asset route is nested under the account as /accounts/:accountId/:assetId and its page is
    // loaded lazily, so waiting for the second segment confirms the click navigated before anything
    // reads the page header. The account page header is the account name, which is otherwise easy
    // to mistake for a balance.
    await this.page.waitForURL(/\/accounts\/[^/]+\/[^/]+/, { timeout: 60_000 });
  }
}
