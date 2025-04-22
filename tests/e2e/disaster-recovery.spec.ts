import { expect, test } from '@playwright/test';
import path from 'path';
import { walletUrl } from './config';
import { AccountAssetPage } from './page-objects/account-asset.page';
import { AccountPage } from './page-objects/account.page';
import { AccountsPage } from './page-objects/accounts.page';
import { DisasterRecoveryPage } from './page-objects/disaster-recovery.page';
import { InitializationPage } from './page-objects/initialization.page';
import { LoginPage } from './page-objects/login.page';
import { SettingsPage } from './page-objects/settings.page';
import { copyArtifact, publishArtifact, topUpAccount } from './utils/orbit.utils';

test('can recover uninstalled station', async ({ page }) => {
  await copyArtifact('station');
  await copyArtifact('upgrader');

  await publishArtifact('station');

  test.setTimeout(600000);

  await page.goto(walletUrl);
  const loginPage = new LoginPage(page);

  await loginPage.register();

  const initializationPage = new InitializationPage(page);
  const stationId = await initializationPage.createStation();

  const accountsPage = new AccountsPage(page);
  await accountsPage.goto();
  await accountsPage.openByName('Main');

  const accountPage = new AccountPage(page);
  await accountPage.pickByAsset('ICP');

  const accountAssetPage = new AccountAssetPage(page);
  const icpNativeAddress = await accountAssetPage.getIcpNativeAddress();

  expect(icpNativeAddress).toBeTruthy();

  await topUpAccount(icpNativeAddress!, 5);

  const settingsPage = new SettingsPage(page);
  settingsPage.go();

  await settingsPage.installCustomWasm(
    path.join(__dirname, '..', '..', 'wasms', 'test_canister.wasm.gz'),
    stationId,
  );

  await page.goto(walletUrl);
  await page.getByText(/disaster recovery/i).click();

  const disasterRecoveryPage = new DisasterRecoveryPage(page);
  await disasterRecoveryPage.assertIsOn();
  await disasterRecoveryPage.waitForLoaded();
  await disasterRecoveryPage.selectRegistryWasm();
  await disasterRecoveryPage.submitRecovery();
  await disasterRecoveryPage.waitRecoverySuccess();

  await accountsPage.goto();
  await accountsPage.openByName('Main');

  await accountPage.pickByAsset('ICP');
  await accountAssetPage.getBalance();

  while (true) {
    // refresh the page
    await page.reload();
    const balance = await accountAssetPage.getBalance();

    if (balance!.includes('5.0')) {
      break;
    }

    await page.waitForTimeout(5000);
  }
});
