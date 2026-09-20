# language: pt
Funcionalidade: Restaurar backup de mundo

  Contexto: O usuário pode restaurar um backup previamente criado,
  substituindo os arquivos atuais do mundo pelo estado do backup.

  Cenário: Listar backups de um mundo
    Dado que o usuário está na lista de mundos
    E o mundo "Mundo Sobrevivência" tem backups
    Quando o usuário clica em "Ver backups" nesse mundo
    Então o comando `cmd_list_backups` deve ser invocado com folder_name e account_id
    E deve exibir lista de backups ordenados por data (mais recente primeiro)
    E cada backup deve mostrar: timestamp formatado, versão do mundo

  Cenário: Restaurar backup com sucesso
    Dado que a lista de backups está visível
    E existe um backup de "2026-09-11T14:30:00Z"
    Quando o usuário clica em "Restaurar" nesse backup
    Então deve exibir modal de confirmação
    Quando o usuário confirma no modal
    Então o comando `cmd_restore_backup` deve ser invocado com backup_path, world_folder_name, account_id
    E deve exibir loading durante a operação
    E ao finalizar deve mostrar toast "Backup restaurado com sucesso"

  Cenário: Cancelar restauração no modal
    Dado que a lista de backups está visível
    Quando o usuário clica em "Restaurar" em um backup
    E o modal de confirmação abre
    Quando o usuário clica em "Cancelar" ou pressiona Escape
    Então o modal deve fechar
    E o comando `cmd_restore_backup` NÃO deve ser invocado

  Cenário: Erro - backup não encontrado
    Dado que a lista de backups está visível
    E o arquivo de backup foi deletado externamente
    Quando o usuário clica em "Restaurar" e confirma
    Então o comando `cmd_restore_backup` retorna erro DomainError::InvalidBackupPath
    E deve exibir toast "Backup não encontrado"
    E deve mostrar botão "Tentar novamente"

  Cenário: Erro - mundo original não encontrado
    Dado que a lista de backups está visível
    E o mundo original foi deletado externamente
    Quando o usuário clica em "Restaurar" e confirma
    Então o comando `cmd_restore_backup` retorna erro DomainError::InvalidWorldPath
    E deve exibir toast "Mundo original não encontrado"
    E deve mostrar botão "Tentar novamente"

  Cenário: Erro - permissão negada no mundo
    Dado que a lista de backups está visível
    E o diretório do mundo não tem permissão de escrita
    Quando o usuário clica em "Restaurar" e confirma
    Então o comando `cmd_restore_backup` retorna erro de I/O (permission denied)
    E deve exibir toast "Sem permissão para restaurar no diretório do mundo"
    E deve mostrar botão "Tentar novamente"

  Cenário: Navegação por teclado na lista de backups
    Dado que a lista de backups está visível
    Quando o usuário navega por Tab
    Então o foco deve mover entre os botões "Restaurar" de cada backup
    E Enter deve abrir o modal de confirmação
    E Escape deve voltar para a lista de mundos

  Cenário: Acessibilidade do modal de confirmação
    Dado que o modal de confirmação está aberto
    Então deve ter role="dialog" e aria-modal="true"
    E o foco deve ser movido para o botão "Cancelar" (padrão seguro)
    E Tab deve circular apenas dentro do modal
    E Escape deve fechar o modal
