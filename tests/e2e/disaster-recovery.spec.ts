import { test, expect } from '@playwright/test';
import { LoginPage } from './page-objects/login';
import { InitializationPage } from './page-objects/initialization';
import { getWalletUrl, walletUrl } from './config';
import { spawnSync } from 'child_process';
import { SettingsPage } from './page-objects/settings';
import path from 'path';

test('can recover uninstalled station', async ({ page }) => {
  test.setTimeout(120000);
  await page.goto(walletUrl);
  const loginPage = new LoginPage(page);

  const _user1Anchor = await loginPage.register();

  const initializationPage = new InitializationPage(page);
  const stationId = await initializationPage.createStation();

  const settingsPage = new SettingsPage(page);
  settingsPage.go();

  await settingsPage.installCustomWasm(
    path.join(__dirname, '..', '..', 'wasms', 'test_canister.wasm.gz'),
    stationId,
  );

  await page.goto(walletUrl);
  await page.getByText(/disaster recovery/i).click();

  await expect(page.url()).toContain('/disaster-recovery');
});
