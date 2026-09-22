import { expect, Page } from '@playwright/test';
import { getWalletPath } from '../config';
import { getCanisterInfo } from '../utils/dfx.utils';

export class SettingsPage {
  constructor(private readonly page: Page) {}

  async go() {
    await this.page.goto(getWalletPath('/en/settings/system'));
  }

  async installCustomWasm(path: string, stationId: string, checkForNewModuleHash: boolean = true) {
    const originalModuleHash = getCanisterInfo(stationId).moduleHash;

    await this.page.getByTestId('submit-upgrade-btn').click();

    await this.page.getByTestId('toggle-form-mode-btn').click();

    const fileChooserPromise = this.page.waitForEvent('filechooser');
    await this.page.getByTestId('advanced-update-mode-wasm-input').click();

    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles(path);

    await this.page.getByTestId('continue-action-btn').click();
    await this.page.getByTestId('submit-action-btn').click();

    if (checkForNewModuleHash) {
      // Bounded so a wasm that never installs fails here rather than consuming the whole test
      // budget and surfacing as a timeout in a later, unrelated step.
      await expect
        .poll(() => getCanisterInfo(stationId).moduleHash, {
          message: 'the station module hash should change after installing the custom wasm',
          timeout: 120_000,
          intervals: [1_000],
        })
        .not.toBe(originalModuleHash);
    }
  }
}
