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

  // The entry point only appears once the wallet has noticed the station is uninstalled, so it is
  // waited for rather than clicked straight away.
  const disasterRecoveryLink = page.getByText(/disaster recovery/i).first();
  await disasterRecoveryLink.waitFor({ state: 'visible', timeout: 60_000 });
  await disasterRecoveryLink.click();

  const disasterRecoveryPage = new DisasterRecoveryPage(page);
  await disasterRecoveryPage.assertIsOn();
  await disasterRecoveryPage.waitForLoaded();
  await disasterRecoveryPage.selectRegistryWasm();
  await disasterRecoveryPage.submitRecovery();
  await disasterRecoveryPage.waitRecoverySuccess();

  await page.waitForTimeout(5000);

  await accountsPage.goto();
  await accountsPage.openByName('Main');

  await accountPage.pickByAsset('ICP');

  await accountAssetPage.expectBalance('5.0');
});
