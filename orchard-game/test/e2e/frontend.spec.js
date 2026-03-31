import { test, expect } from '@playwright/test';

test.describe('Orchard Game Frontend', () => {
  test('homepage loads correctly', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/Orchard/);
  });

  test('navigation menu works', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Plant');
    await expect(page.url()).toContain('/plant');
  });

  test('seed planting form displays', async ({ page }) => {
    await page.goto('/plant');
    await expect(page.locator('input[name="payload"]')).toBeVisible();
    await expect(page.locator('input[name="stake"]')).toBeVisible();
  });

  test('federation page loads', async ({ page }) => {
    await page.goto('/federations');
    await expect(page.locator('text=Create Federation')).toBeVisible();
  });

  test('leaderboard page loads', async ({ page }) => {
    await page.goto('/leaderboard');
    await expect(page.locator('text=Leaderboard')).toBeVisible();
  });

  test('duel page loads', async ({ page }) => {
    await page.goto('/duel');
    await expect(page.locator('text=Initiate Duel')).toBeVisible();
  });
});
