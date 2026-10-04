import { defineConfig, devices } from '@playwright/test';

declare const process: {
  env: Record<string, string | undefined>;
};

const isCI = !!process.env.CI;
const tauriPort = 9222;
const tauriUrl = `http://localhost:${tauriPort}`;

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
        connectOptions: {
          wsEndpoint: `ws://localhost:${tauriPort}/devtools/browser`,
        },
      },
    },
  ],
  webServer: isCI ? {
    command: 'cargo tauri build --manifest-path ../src-tauri/Cargo.toml',
    url: tauriUrl,
    reuseExistingServer: false,
    timeout: 300_000,
    cwd: '..',
    env: {
      BLOCKORIA_TEST_WORLDS_DIR: process.env.BLOCKORIA_TEST_WORLDS_DIR || '',
      WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${tauriPort}`,
    },
  } : {
    command: 'cargo tauri dev --manifest-path ../src-tauri/Cargo.toml',
    url: tauriUrl,
    reuseExistingServer: !process.env.CI_FORCE_REBUILD,
    timeout: 180_000,
    env: {
      WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${tauriPort}`,
      BLOCKORIA_TEST_WORLDS_DIR: process.env.BLOCKORIA_TEST_WORLDS_DIR || '',
    },
  },
});
