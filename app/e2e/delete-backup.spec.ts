import { test, expect } from '@playwright/test';

test.describe('Delete Backup', () => {
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

  test('should delete backup', async ({ page }) => {
    await expect(page.locator('h2:has-text("Backups")')).toBeVisible();

    // Click trash icon on the backup
    const backup = page.locator('article[role="listitem"]').first();
    await backup.locator('button[aria-label^="Deletar backup"]').click();

    // Confirm deletion in modal
    await expect(page.locator('h3:has-text("Confirmar Exclusão")')).toBeVisible();
    await page.locator('button:has-text("Deletar")').last().click();

    // Should show success toast and backup list should be empty
    await expect(page.locator('text=Backup deletado com sucesso')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('article[role="listitem"]')).toHaveCount(0);
    await expect(page.locator('text=Nenhum backup encontrado')).toBeVisible();
  });
});
