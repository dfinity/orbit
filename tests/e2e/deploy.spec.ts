import { test, expect } from '@playwright/test';
import { LoginPage } from './page-objects/login';
import { InitializationPage } from './page-objects/initialization';
import { walletUrl } from './config';

test('can deploy station', async ({ page }) => {
  test.setTimeout(120000);

  await page.goto(walletUrl);

  const loginPage = new LoginPage(page);
  await loginPage.assertOnLoginPage();
  await loginPage.register();

  await expect(page).toHaveURL(/initialization/);

  const initializationPage = new InitializationPage(page);
  await initializationPage.createStation();
});
