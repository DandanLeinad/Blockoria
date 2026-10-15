import { defineConfig, devices } from '@playwright/test';

declare const process: {
  env: Record<string, string | undefined>;
};

const isCI = !!process.env.CI;
const tauriHost = '127.0.0.1';
const tauriPort = 9222;
const tauriUrl = `http://${tauriHost}:${tauriPort}`;
const cdpUrl = `http://${tauriHost}:${tauriPort}`;

export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  retries: isCI ? 1 : 0,
  workers: isCI ? 1 : undefined,
  reporter: [['html', { outputFolder: 'playwright-report' }]],
  use: {
    baseURL: tauriUrl,
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
  },
  projects: [
    {
      name: 'tauri-chromium',
      use: {
        ...devices['Desktop Chrome'],
      },
    },
  ],
  webServer: {
    command: 'cargo tauri dev --no-watch',
    url: `${tauriUrl}/json/version`,
    reuseExistingServer: false,
    timeout: 600_000,
    cwd: '..',
    stdout: 'pipe',
    stderr: 'pipe',
env: {
      BLOCKORIA_TEST_WORLDS_DIR: process.env.BLOCKORIA_TEST_WORLDS_DIR || '',
      WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-address=${tauriHost} --remote-debugging-port=${tauriPort}`,
    },
  },
  },
});
