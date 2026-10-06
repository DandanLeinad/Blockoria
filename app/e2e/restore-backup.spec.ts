import { test, expect } from '@playwright/test';

test.describe('Restore Backup', () => {
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

  test('should restore backup', async ({ page }) => {
    await expect(page.locator('h2:has-text("Backups")')).toBeVisible();

    // Click "Restaurar" on the backup
    const backup = page.locator('article[role="listitem"]').first();
    await backup.locator('button:has-text("Restaurar")').click();

    // Confirm restoration in modal
    await expect(page.locator('h3:has-text("Confirmar Restauração")')).toBeVisible();
    await expect(page.locator('text=Test World 1')).toBeVisible();
    await page.locator('button:has-text("Restaurar")').last().click();

    // Should show success toast and go back to world list
    await expect(page.locator('text=Backup restaurado com sucesso')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('h2:has-text("Mundos")')).toBeVisible();
  });
});
