export function formatVersion(v: [number, number, number, number, number]): string {
  return v.join('.')
}

export function formatDate(iso: string): string {
  const date = new Date(iso)
  return date.toLocaleString('pt-BR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  })
}
