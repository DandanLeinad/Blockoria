import { test, expect } from '@playwright/test';

test.describe('List Backups', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('article[role="listitem"]')).toHaveCount(3);

    // Create a backup first
    const firstWorld = page.locator('article[role="listitem"]').first();
    await firstWorld.locator('button:has-text("Criar backup")').click();
    await expect(page.locator('h2:has-text("Criar Backup")')).toBeVisible({ timeout: 10000 });
    await page.locator('button:has-text("Criar Backup")').last().click();
    await expect(page.locator('h2:has-text("Backup criado com sucesso")')).toBeVisible({ timeout: 30000 });
    await page.locator('button:has-text("Ver backups")').click();
  });

  test('should display list of backups', async ({ page }) => {
    await expect(page.locator('h2:has-text("Backups")')).toBeVisible();
    await expect(page.locator('text=Test World 1')).toBeVisible();

    // Should have at least 1 backup
    const backupCards = page.locator('article[role="listitem"]');
    await expect(backupCards).toHaveCount(1);

    // Check backup details
    const backup = backupCards.first();
    await expect(backup.locator('text=Versão:')).toBeVisible();
    await expect(backup.locator('code.font-mono')).toContainText('1.21.0.0');
  });

  test('should show backup icon', async ({ page }) => {
    const backup = page.locator('article[role="listitem"]').first();
    const icon = backup.locator('img').first();
    await expect(icon).toBeVisible();
  });
});
