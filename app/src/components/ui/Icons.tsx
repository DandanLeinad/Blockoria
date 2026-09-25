import * as Lucide from 'lucide-react'

export const Icons = {
  folderOpen: Lucide.FolderOpen,
  arrowLeft: Lucide.ArrowLeft,
  refreshCw: Lucide.RefreshCw,
  plus: Lucide.Plus,
  trash2: Lucide.Trash2,
  rotateCcw: Lucide.RotateCcw,
  alertTriangle: Lucide.AlertTriangle,
  checkCircle: Lucide.CheckCircle,
  xCircle: Lucide.XCircle,
  info: Lucide.Info,
  database: Lucide.Database,
  clock: Lucide.Clock,
  chevronDown: Lucide.ChevronDown,
  menu: Lucide.Menu,
  x: Lucide.X,
  externalLink: Lucide.ExternalLink,
  download: Lucide.Download,
  upload: Lucide.Upload,
  settings: Lucide.Settings,
  home: Lucide.Home,
  search: Lucide.Search,
  eye: Lucide.Eye,
  eyeOff: Lucide.EyeOff,
  copy: Lucide.Copy,
  edit: Lucide.Edit,
  save: Lucide.Save,
  loader: Lucide.Loader2,
  alertCircle: Lucide.AlertCircle,
  check: Lucide.Check,
  moon: Lucide.Moon,
  sun: Lucide.Sun,
  monitor: Lucide.Monitor,
  users: Lucide.Users,
  user: Lucide.User,
  fileText: Lucide.FileText,
} as const

export type IconName = keyof typeof Icons

export function Icon({
  name,
  className = 'w-5 h-5',
  ...props
}: {
  name: IconName;
  className?: string;
} & React.SVGProps<SVGSVGElement>) {
  const Component = Icons[name]
  return <Component className={className} aria-hidden="true" {...props} />
}
