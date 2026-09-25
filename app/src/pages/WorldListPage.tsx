import { useNavigate } from 'react-router-dom'
import type { WorldSummaryDto } from '../components/WorldList'
import { WorldList } from '../components/WorldList'

export function WorldListPage() {
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
        navigate(`/world/${encodeURIComponent(world.folder_name)}/backup/create`, { state: { levelName: world.level_name } })
      }}
      onViewBackups={(world) => navigate(getBackupsPath(world), { state: { levelName: world.level_name } })}
    />
  )
}
