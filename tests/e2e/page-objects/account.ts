import { Page } from '@playwright/test';

export class AccountPage {
  constructor(private page: Page) {}

  async pickByAsset(asset: string) {
    await this.page.getByTestId('account-assets-table').getByText(asset).click();
  }
}
