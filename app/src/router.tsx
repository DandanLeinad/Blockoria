import { createBrowserRouter, Link, NavLink, Outlet, useNavigate, useParams } from 'react-router-dom'
import { CreateBackup } from './components/CreateBackup'
import { ListBackups } from './components/ListBackups'
import type { WorldSummaryDto } from './components/WorldList'
import { WorldList } from './components/WorldList'

export const router = createBrowserRouter([
  {
    path: '/',
    element: <AppLayout />,
    children: [
      {
        index: true,
        element: <WorldListPage />,
      },
      {
        path: 'world/:folderName/backup/create',
        element: <CreateBackupPage />,
      },
      {
        path: 'world/:folderName/backups',
        element: <ListBackupsPage />,
      },
    ],
  },
])

function AppLayout() {

  const navItems = [
    { path: '/', label: 'Mundos', icon: (
      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 21v-6m0 0v-6m0 6H7m4 0h4" />
      </svg>
    )},
  ]

  return (
    <div className="min-h-screen bg-background flex">
      {/* Sidebar */}
      <aside className="w-64 bg-card border-r border-border flex flex-col hidden lg:flex">
        <div className="p-4 border-b border-border">
          <Link to="/" className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-lg bg-primary flex items-center justify-center">
              <svg className="w-5 h-5 text-primary-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
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

        <div className="p-4 border-t border-border">
          <div className="text-xs text-muted-foreground text-center">
            v0.6.0
          </div>
        </div>
      </aside>

      {/* Mobile header */}
      <header className="lg:hidden bg-card border-b border-border sticky top-0 z-40">
        <div className="px-4 py-3 flex items-center justify-between">
          <Link to="/" className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-lg bg-primary flex items-center justify-center">
              <svg className="w-5 h-5 text-primary-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
              </svg>
            </div>
            <span className="font-bold text-lg text-foreground">Blockoria</span>
          </Link>
        </div>
      </header>

      {/* Main content */}
      <main className="flex-1 min-w-0">
        <div className="max-w-6xl mx-auto p-4 lg:p-6">
          <Outlet />
        </div>
      </main>
    </div>
  )
}

function WorldListPage() {
  const navigate = useNavigate()

  const getBackupsPath = (world: WorldSummaryDto) => {
    const accountQuery = world.account_id
      ? `?accountId=${encodeURIComponent(world.account_id)}`
      : ''
    return `/world/${encodeURIComponent(world.folder_name)}/backups${accountQuery}`
  }

  return (
    <WorldList
      onWorldSelect={(world) => {
        navigate(`/world/${encodeURIComponent(world.folder_name)}/backup/create`)
      }}
      onViewBackups={(world) => navigate(getBackupsPath(world))}
    />
  )
}

function CreateBackupPage() {
  const params = useParams()
  const navigate = useNavigate()

  return (
    <CreateBackup
      folderName={params.folderName!}
      onClose={() => navigate('/')}
      onSuccess={(world) => navigate(`/world/${encodeURIComponent(world.folder_name)}/backups${world.account_id ? `?accountId=${encodeURIComponent(world.account_id)}` : ''}`)}
    />
  )
}

function ListBackupsPage() {
  const params = useParams()
  const navigate = useNavigate()
  const accountId = new URLSearchParams(window.location.search).get('accountId')

  return (
    <ListBackups
      worldFolderName={params.folderName!}
      accountId={accountId}
      onClose={() => navigate('/')}
    />
  )
}
