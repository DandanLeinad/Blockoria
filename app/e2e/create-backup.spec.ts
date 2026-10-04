import { test, expect } from '@playwright/test';

test.describe('Create Backup', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('article[role="listitem"]')).toHaveCount(3);
  });

  test('should create backup for first world', async ({ page }) => {
    // Click "Criar backup" on first world
    const firstWorld = page.locator('article[role="listitem"]').first();
    await firstWorld.locator('button:has-text("Criar backup")').click();

    // Wait for CreateBackup modal to load
    await expect(page.locator('h2:has-text("Criar Backup")')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Test World 1')).toBeVisible();

    // Click "Criar Backup" button
    await page.locator('button:has-text("Criar Backup")').last().click();

    // Wait for success state
    await expect(page.locator('h2:has-text("Backup criado com sucesso")')).toBeVisible({ timeout: 30000 });
    await expect(page.locator('text=Test World 1')).toBeVisible();
    await expect(page.locator('font-mono:has-text("backup")')).toBeVisible();

    // Click "Ver backups" to navigate to list backups
    await page.locator('button:has-text("Ver backups")').click();

    // Should be in ListBackups view with the new backup
    await expect(page.locator('h2:has-text("Backups")')).toBeVisible();
    await expect(page.locator('article[role="listitem"]')).toHaveCount(1);
  });

  test('should create backup for shared world', async ({ page }) => {
    // Click "Criar backup" on second world (Shared World)
    const secondWorld = page.locator('article[role="listitem"]').nth(1);
    await secondWorld.locator('button:has-text("Criar backup")').click();

    await expect(page.locator('h2:has-text("Criar Backup")')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Shared World')).toBeVisible();
    await expect(page.locator('text=Compartilhado')).toBeVisible();

    await page.locator('button:has-text("Criar Backup")').last().click();

    await expect(page.locator('h2:has-text("Backup criado com sucesso")')).toBeVisible({ timeout: 30000 });
  });
});
