import { test as base, chromium, type Page } from '@playwright/test';

export const test = base.extend<{ page: Page }>({
  page: async ({}, use) => {
    const browser = await chromium.connectOverCDP('http://127.0.0.1:9222');
    const context = browser.contexts()[0];
    const page = context.pages()[0] ?? (await context.waitForEvent('page'));
    await use(page);
    await browser.close();
  },
});

export { expect } from '@playwright/test';
