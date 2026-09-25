import { createBrowserRouter } from 'react-router-dom'
import { ToastProvider } from './components/Toast'
import { ThemeProvider } from './components/Theme'
import { WorldListPage } from './pages/WorldListPage'
import { CreateBackupPage } from './pages/CreateBackupPage'
import { ListBackupsPage } from './pages/ListBackupsPage'
import { AppLayout } from './components/AppLayout'

export const router = createBrowserRouter([
  {
    path: '/',
    element: (
      <ThemeProvider>
        <ToastProvider>
          <AppLayout />
        </ToastProvider>
      </ThemeProvider>
    ),
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
