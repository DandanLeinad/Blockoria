import { test, expect } from '@playwright/test';

test.describe('Full Flow: List → Create → List → Restore → Delete', () => {
  test('complete backup lifecycle', async ({ page }) => {
    // ===== LIST WORLDS =====
    await page.goto('/');
    await expect(page.locator('article[role="listitem"]')).toHaveCount(3);

    // Verify all three worlds are visible
    const worlds = page.locator('article[role="listitem"]');
    await expect(worlds.first().locator('h3')).toContainText('Test World 1');
    await expect(worlds.nth(1).locator('h3')).toContainText('Shared World');
    await expect(worlds.nth(2).locator('h3')).toContainText('Restore Test World');

    // ===== CREATE BACKUP =====
    await worlds.first().locator('button:has-text("Criar backup")').click();
    await expect(page.locator('h2:has-text("Criar Backup")')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Test World 1')).toBeVisible();

    await page.locator('button:has-text("Criar Backup")').last().click();
    await expect(page.locator('h2:has-text("Backup criado com sucesso")')).toBeVisible({ timeout: 30000 });
    await expect(page.locator('text=Test World 1')).toBeVisible();

    // ===== LIST BACKUPS =====
    await page.locator('button:has-text("Ver backups")').click();
    await expect(page.locator('h2:has-text("Backups")')).toBeVisible();
    await expect(page.locator('text=Test World 1')).toBeVisible();

    // Verify backup exists
    let backups = page.locator('article[role="listitem"]');
    await expect(backups).toHaveCount(1);

    const backup = backups.first();
    await expect(backup.locator('code.font-mono')).toContainText('1.21.0.0');

    // ===== RESTORE BACKUP =====
    await backup.locator('button:has-text("Restaurar")').click();
    await expect(page.locator('h3:has-text("Confirmar Restauração")')).toBeVisible();
    await page.locator('button:has-text("Restaurar")').last().click();

    // Success toast and return to world list
    await expect(page.locator('text=Backup restaurado com sucesso')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('h2:has-text("Mundos")')).toBeVisible();

    // ===== CREATE ANOTHER BACKUP FOR DELETE TEST =====
    const firstWorldAgain = page.locator('article[role="listitem"]').first();
    await firstWorldAgain.locator('button:has-text("Criar backup")').click();
    await expect(page.locator('h2:has-text("Criar Backup")')).toBeVisible({ timeout: 10000 });
    await page.locator('button:has-text("Criar Backup")').last().click();
    await expect(page.locator('h2:has-text("Backup criado com sucesso")')).toBeVisible({ timeout: 30000 });
    await page.locator('button:has-text("Ver backups")').click();

    // ===== LIST BACKUPS (verify second backup exists) =====
    await expect(page.locator('h2:has-text("Backups")')).toBeVisible();
    backups = page.locator('article[role="listitem"]');
    await expect(backups).toHaveCount(1);

    // ===== DELETE BACKUP =====
    const backupToDelete = backups.first();
    await backupToDelete.locator('button[aria-label^="Deletar backup"]').click();
    await expect(page.locator('h3:has-text("Confirmar Exclusão")')).toBeVisible();
    await page.locator('button:has-text("Deletar")').last().click();

    // Success toast and empty list
    await expect(page.locator('text=Backup deletado com sucesso')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('article[role="listitem"]')).toHaveCount(0);
    await expect(page.locator('text=Nenhum backup encontrado')).toBeVisible();

    // ===== BACK TO WORLD LIST =====
    await page.locator('button[aria-label="Voltar"]').click();
    await expect(page.locator('h2:has-text("Mundos")')).toBeVisible();
    await expect(page.locator('article[role="listitem"]')).toHaveCount(3);
  });
});
