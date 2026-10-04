import { test, expect } from '@playwright/test';

test.describe('List Worlds', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('should display list of worlds', async ({ page }) => {
    // Wait for worlds to load
    await expect(page.locator('[role="list"][aria-label="Lista de mundos do Minecraft Bedrock"]')).toBeVisible();

    // Should have 3 test worlds
    const worldCards = page.locator('article[role="listitem"]');
    await expect(worldCards).toHaveCount(3);

    // Check first world (Test World 1)
    const firstWorld = worldCards.first();
    await expect(firstWorld.locator('h3')).toContainText('Test World 1');
    await expect(firstWorld.locator('code.font-mono')).toContainText('1.21.0.0');
    await expect(firstWorld.locator('span:has-text("Conta")')).toBeVisible();

    // Check second world (Shared World)
    const secondWorld = worldCards.nth(1);
    await expect(secondWorld.locator('h3')).toContainText('Shared World');
    await expect(secondWorld.locator('code.font-mono')).toContainText('1.20.80.0');
    await expect(secondWorld.locator('span:has-text("Compartilhado")')).toBeVisible();

    // Check third world (Restore Test World)
    const thirdWorld = worldCards.nth(2);
    await expect(thirdWorld.locator('h3')).toContainText('Restore Test World');
    await expect(thirdWorld.locator('code.font-mono')).toContainText('1.21.10.0');
  });

  test('should show world icon', async ({ page }) => {
    const worldCards = page.locator('article[role="listitem"]');
    const firstWorld = worldCards.first();

    const icon = firstWorld.locator('img').first();
    await expect(icon).toBeVisible();
    await expect(icon).toHaveAttribute('src', /data:image\/jpeg;base64,/);
  });
});
