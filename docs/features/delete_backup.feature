# language: pt
Funcionalidade: Deletar backup

  Contexto: O usuário pode deletar backups individuais da lista.
  A ação é irreversível e requer confirmação.

  Cenário: Deletar backup com sucesso
    Dado que a lista de backups está visível
    E existe um backup de "2026-09-11T14:30:00Z"
    Quando o usuário clica no botão "Deletar" (ícone 🗑️) nesse backup
    Então deve exibir modal de confirmação
    Quando o usuário confirma no modal
    Então o comando `cmd_delete_backup` deve ser invocado com backup_path
    E deve exibir loading no botão durante a operação
    E ao finalizar deve remover o item da lista
    E deve mostrar toast "Backup deletado com sucesso"

  Cenário: Cancelar deleção no modal
    Dado que a lista de backups está visível
    Quando o usuário clica em "Deletar" em um backup
    E o modal de confirmação abre
    Quando o usuário clica em "Cancelar" ou pressiona Escape
    Então o modal deve fechar
    E o comando `cmd_delete_backup` NÃO deve ser invocado
    E o item deve permanecer na lista

  Cenário: Erro - backup não encontrado (idempotente)
    Dado que a lista de backups está visível
    E o arquivo de backup foi deletado externamente
    Quando o usuário clica em "Deletar" e confirma
    Então o comando `cmd_delete_backup` retorna sucesso (idempotente)
    E deve remover o item da lista
    E deve mostrar toast "Backup deletado com sucesso"

  Cenário: Erro - permissão negada
    Dado que a lista de backups está visível
    E o arquivo de backup não tem permissão de exclusão
    Quando o usuário clica em "Deletar" e confirma
    Então o comando `cmd_delete_backup` retorna erro de I/O (permission denied)
    E deve exibir toast "Sem permissão para deletar este backup"
    E o item deve permanecer na lista
    E deve mostrar botão "Tentar novamente"

  Cenário: Deletar último backup → estado vazio
    Dado que a lista de backups tem apenas 1 item
    Quando o usuário deleta esse backup com sucesso
    Então a lista deve ficar vazia
    E deve exibir estado vazio "Nenhum backup encontrado para este mundo"

  Cenário: Navegação por teclado - botão deletar
    Dado que a lista de backups está visível
    Quando o usuário navega por Tab
    Então o foco deve alcançar o botão "Deletar" (ícone) de cada backup
    E Enter/Space deve abrir o modal de confirmação
    E o botão deve ter aria-label descritivo
