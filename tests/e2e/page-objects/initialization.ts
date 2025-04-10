import { expect, Page } from '@playwright/test';
import exp from 'constants';

export class InitializationPage {
  constructor(private page: Page) {}

  async createStation(
    walletName: string = 'wallet',
    adminName: string = 'adminuser',
  ): Promise<string> {
    await this.page.getByTestId('deploy-new-station-radio').locator('input').click();
    await this.page.getByTestId('continue-button').click();

    await this.page
      .locator('[data-test-id="deploy-station-form-name-field"] input')
      .fill(walletName);
    await this.page
      .locator('[data-test-id="deploy-station-form-admin-name-field"] input')
      .fill(adminName);

    await this.page.getByTestId('deploy-station-form-continue-button').click();

    await expect(this.page).toHaveURL(/dashboard/, { timeout: 50000 });

    // get url params
    const urlParams = new URL(this.page.url()).searchParams;
    const canisterId = urlParams.get('stationId');

    expect(canisterId).toBeDefined();

    return canisterId!;
  }

  async joinStation(canisterId: string) {
    await this.page.getByTestId('join-existing-station-radio').click();
    await this.page.getByTestId('continue-button').click();
  }
}
