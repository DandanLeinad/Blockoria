import { Link, NavLink, Outlet } from 'react-router-dom'
import { Icon } from './ui/Icons'
import { Toaster } from './ui/toast'

export function AppLayout() {

  const navItems = [
    { path: '/', label: 'Mundos', icon: <Icon name="folderOpen" className="w-5 h-5" /> },
    { path: '/settings', label: 'Configurações', icon: <Icon name="settings" className="w-5 h-5" /> },
  ]

  return (
    <div className="min-h-screen bg-background flex">
      {/* Sidebar */}
      <aside className="w-64 bg-card border-r border-border flex flex-col hidden lg:flex">
        <div className="p-4 border-b border-border">
          <Link to="/" className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-lg bg-primary flex items-center justify-center">
              <svg className="w-5 h-5 text-primary-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 110 4m-6 8a2 2 0 110-4m0 4a2 2 0 110-4m0 4a2 2 0 110-4m0 4a2 2 0 110-4m0 4a2 2 0 110-4m0 4a2 2 0 110-4m0 4a2 2 0 110-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 110-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
              </svg>
            </div>
            <div>
              <h1 className="font-bold text-lg text-foreground">Blockoria</h1>
              <p className="text-xs text-muted-foreground">Minecraft Bedrock Backup Manager</p>
            </div>
          </Link>
        </div>

        <nav className="flex-1 p-4 space-y-1 overflow-y-auto">
          {navItems.map((item) => (
            <NavLink
              key={item.path}
              to={item.path}
              className={({ isActive }) =>
                `flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                  isActive
                    ? 'bg-primary text-primary-foreground shadow-sm'
                    : 'text-muted-foreground hover:bg-muted hover:text-foreground'
                }`
              }
            >
              {item.icon}
              {item.label}
            </NavLink>
          ))}
        </nav>
      </aside>

      {/* Mobile header */}
      <header className="lg:hidden bg-card border-b border-border sticky top-0 z-40">
        <div className="px-4 py-3 flex items-center justify-between">
          <Link to="/" className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-lg bg-primary flex items-center justify-center">
              <svg className="w-5 h-5 text-primary-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m0-4a2 2 0 110-4m0 4a2 2 0 110-4m0 4a2 2 0 110-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 110-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 110-4m0 4v2m0-6V4" />
              </svg>
            </div>
            <div className="font-bold text-lg text-foreground">Blockoria</div>
          </Link>
        </div>
      </header>

      {/* Main content */}
      <main className="flex-1 min-w-0">
        <div className="max-w-6xl mx-auto p-4 lg:p-6">
          <Outlet />
        </div>
      </main>
      <Toaster position="top-right" richColors />
    </div>
  )
}
