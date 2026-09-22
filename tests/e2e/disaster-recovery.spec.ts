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
import { getStationHealthStatus } from './utils/dfx.utils';
import { copyArtifact, publishArtifact, topUpAccount } from './utils/orbit.utils';

test('can recover uninstalled station', async ({ page }) => {
  test.setTimeout(600000);

  await copyArtifact('station');
  await copyArtifact('upgrader');

  await publishArtifact('station');

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

  // replace the station module with an unrelated canister, the wallet can no longer connect to it
  const settingsPage = new SettingsPage(page);
  await settingsPage.go();

  await settingsPage.installCustomWasm(
    path.join(__dirname, '..', '..', 'wasms', 'test_canister.wasm.gz'),
    stationId,
  );

  // the wallet fails to connect to the station and shows the error screen with the disaster
  // recovery link
  await page.goto(walletUrl);

  const disasterRecoveryPage = new DisasterRecoveryPage(page);
  await disasterRecoveryPage.openFromErrorScreen();
  await disasterRecoveryPage.selectRegistryWasm();
  await disasterRecoveryPage.submitRecovery();
  await disasterRecoveryPage.waitRecoverySuccess();

  // the recovered station finishes its initialization (e.g. adding the accounts) in a timer after
  // the installation and rejects calls until then, wait for it to be healthy before connecting
  await expect
    .poll(() => getStationHealthStatus(stationId), { timeout: 120_000, intervals: [1_000] })
    .toContain('Healthy');

  // the recovered station has the same accounts and fetches their balances again from the ledger
  await accountsPage.goto();
  await accountsPage.openByName('Main');

  await accountPage.pickByAsset('ICP');
  await accountAssetPage.expectBalance('5.0');
});
