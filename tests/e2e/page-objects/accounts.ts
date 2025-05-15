import { Page } from '@playwright/test';
import { getWalletUrl } from '../config';

export class AccountsPage {
  constructor(private page: Page) {}

  async goto() {
    await this.page.goto(getWalletUrl('/en/accounts'));
  }

  async openByName(name: string) {
    await this.page.getByTestId('accounts-table').getByText(name).click();
  }
}
