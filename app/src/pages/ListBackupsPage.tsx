import { useNavigate, useParams, useLocation } from 'react-router-dom'
import { ListBackups } from '../components/ListBackups'

export function ListBackupsPage() {
  const params = useParams()
  const navigate = useNavigate()
  const location = useLocation()
  const accountId = new URLSearchParams(window.location.search).get('accountId')
  const levelName = (location.state as { levelName?: string })?.levelName

  return (
    <ListBackups
      worldFolderName={params.folderName!}
      levelName={levelName}
      accountId={accountId}
      onClose={() => navigate('/')}
    />
  )
}
