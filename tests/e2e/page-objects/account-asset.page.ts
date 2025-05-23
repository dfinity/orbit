import { Page } from '@playwright/test';

export class AccountAssetPage {
  constructor(private page: Page) {}

  async getIcpNativeAddress() {
    return await this.page.getByTestId('icp_account_identifier').getAttribute('title');
  }

  async getBalance() {
    return await this.page.getByTestId('page-header-title').textContent();
  }
}
