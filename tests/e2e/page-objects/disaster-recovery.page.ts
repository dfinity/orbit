import { expect, Page } from '@playwright/test';

export class DisasterRecoveryPage {
  constructor(private page: Page) {}

  /**
   * Opens the disaster recovery page through the link shown on the error screen that the wallet
   * renders when it cannot connect to the station.
   *
   * The click is retried until the page has landed on the disaster recovery route and rendered the
   * upgrader state, so a navigation that is still settling right after the page load cannot leave
   * the test stranded on the error screen.
   */
  async openFromErrorScreen() {
    await expect(async () => {
      if (!/disaster-recovery/.test(this.page.url())) {
        await this.page
          .getByRole('link', { name: /disaster recovery/i })
          .click({ timeout: 15_000 });
      }

      await this.assertIsOn(5_000);
      await this.waitForLoaded(30_000);
    }).toPass({ timeout: 180_000 });
  }

  async assertIsOn(timeout = 50_000) {
    await expect(this.page).toHaveURL(/disaster-recovery/, { timeout });
  }

  async waitForLoaded(timeout = 60_000) {
    await expect(this.page.getByText(/Disaster Recovery State/i)).toBeVisible({ timeout });
  }

  async selectRegistryWasm() {
    await this.page.getByTestId('select-registry-wasm').getByRole('combobox').click();
    await this.page.getByRole('option').click();
  }

  async submitRecovery() {
    const submitButton = this.page.getByTestId('submit-recovery-button');

    await submitButton.click();
    await expect(submitButton).toHaveAttribute('disabled');
    await expect(submitButton).not.toHaveAttribute('disabled', { timeout: 60_000 });
  }

  async waitRecoverySuccess(timeout = 180_000) {
    await expect(this.page.getByText(/Disaster recovery succeeded/i)).toBeVisible({ timeout });
  }
}
