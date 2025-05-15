import { expect, Page } from '@playwright/test';

export class DisasterRecoveryPage {
  constructor(private page: Page) {}

  async assertIsOn() {
    await expect(this.page).toHaveURL(/disaster-recovery/, { timeout: 50000 });
  }

  async waitForLoaded() {
    await this.page.getByText(/Disaster Recovery State/i).waitFor({ state: 'visible' });
  }

  async selectRegistryWasm() {
    await this.page.getByTestId('select-registry-wasm').getByRole('combobox').click();
    await this.page.getByRole('option').click();
  }

  async submitRecovery() {
    await this.page.getByTestId('submit-recovery-button').click();
    await expect(this.page.getByTestId('submit-recovery-button')).toHaveAttribute('disabled');
    await expect(this.page.getByTestId('submit-recovery-button')).not.toHaveAttribute('disabled');
  }

  async waitRecoverySuccess() {
    await this.page.getByText(/Disaster recovery succeeded/i).waitFor({ state: 'visible' });
  }
}
