import { useNavigate, useParams, useLocation } from 'react-router-dom'
import { CreateBackup } from '../components/CreateBackup'

export function CreateBackupPage() {
  const params = useParams()
  const navigate = useNavigate()
  const location = useLocation()
  const levelName = (location.state as { levelName?: string })?.levelName

  return (
    <CreateBackup
      folderName={params.folderName!}
      levelName={levelName}
      onClose={() => navigate('/')}
      onSuccess={(world) => navigate(`/world/${encodeURIComponent(world.folder_name)}/backups${world.account_id ? `?accountId=${encodeURIComponent(world.account_id)}` : ''}`, { state: { levelName: world.level_name } })}
    />
  )
}
