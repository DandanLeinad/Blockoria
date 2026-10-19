# language: pt
Funcionalidade: Criar backup de mundo

  Contexto: O usuário pode criar um backup versionado de qualquer mundo
  listado, que será salvo no diretório de backups configurado com
  timestamp no nome da pasta.

  Cenário: Criar backup com sucesso
    Dado que a lista de mundos está visível
    E o usuário seleciona o mundo "Mundo Sobrevivência" (compartilhado)
    Quando o usuário clica em "Criar backup"
    Então o comando `cmd_create_backup` deve ser invocado com folder_name e account_id=null
    E deve exibir loading durante a operação
    E ao finalizar deve mostrar toast "Backup criado com sucesso"
    E deve mostrar o caminho do backup e timestamp

  Cenário: Criar backup de mundo de conta Xbox
    Dado que a lista de mundos está visível
    E o usuário seleciona o mundo "Creative World" (conta abc123def456)
    Quando o usuário clica em "Criar backup"
    Então o comando `cmd_create_backup` deve ser invocado com account_id="abc123def456"
    E deve exibir loading durante a operação
    E ao finalizar deve mostrar toast de sucesso

  Cenário: Erro - mundo não encontrado
    Dado que o usuário seleciona um mundo que foi deletado externamente
    Quando o usuário clica em "Criar backup"
    Então o comando `cmd_create_backup` retorna erro DomainError::InvalidWorldPath
    E deve exibir toast "Mundo não encontrado"
    E deve mostrar botão "Tentar novamente"

  Cenário: Erro - permissão negada no diretório de backup
    Dado que o diretório de backup não tem permissão de escrita
    Quando o usuário clica em "Criar backup"
    Então o comando `cmd_create_backup` retorna erro de I/O
    E deve exibir toast "Sem permissão para escrever no diretório de backup"
    E deve mostrar botão "Tentar novamente"

  Cenário: Erro - disco cheio
    Dado que o disco está cheio
    Quando o usuário clica em "Criar backup"
    Então o comando `cmd_create_backup` retorna erro de I/O (disk full)
    E deve exibir toast "Espaço insuficiente em disco"
    E deve mostrar botão "Tentar novamente"

  Cenário: Cancelar criação de backup
    Dado que a tela de criar backup está aberta
    Quando o usuário clica em "Cancelar" ou pressiona Escape
    Então deve voltar para a lista de mundos
    E não deve invocar `cmd_create_backup`

  Cenário: Navegação por teclado
    Dado que a tela de criar backup está aberta
    Quando o usuário navega por Tab entre os elementos
    Então o foco deve mover: botão Cancelar → botão Criar backup
    E Enter no botão Criar backup deve disparar a ação
    E Escape deve fechar/cancelar
