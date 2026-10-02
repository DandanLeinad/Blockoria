import { vi } from 'vitest'
import '@testing-library/jest-dom'

// Mock requestAnimationFrame for tests
global.requestAnimationFrame = vi.fn(cb => setTimeout(cb, 16))
global.cancelAnimationFrame = vi.fn(id => clearTimeout(id))

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

// Mock Tauri plugins
vi.mock('@tauri-apps/plugin-dialog', () => ({
  open: vi.fn(),
  save: vi.fn(),
  message: vi.fn(),
}))

vi.mock('@tauri-apps/plugin-fs', () => ({
  readDir: vi.fn(),
  readFile: vi.fn(),
  writeFile: vi.fn(),
}))

vi.mock('@tauri-apps/plugin-opener', () => ({
  openPath: vi.fn(),
  openUrl: vi.fn(),
}))
