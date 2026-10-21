# encoding: utf-8
# language: pt

@erros
Funcionalidade: Taxonomia de erros e contrato IPC
  Como desenvolvedor do Blockoria
  Eu quero categorias de erro honestas por camada e um contrato estruturado no IPC
  Para que o frontend exiba mensagens corretas e o sistema saiba o que falhou

  Contexto:
    O tipo de erro dos ports passa a ser ApplicationError
    Categorias: NotFound { what }, Validation (DomainError), Storage (io::Error), Config
    O contrato IPC entrega { "code", "message" } ao frontend
    Codes: not_found | validation | storage | config | unknown

  @not-found @application
  Cenário: Restaurar com backup inexistente retorna NotFound
    Dado um repositório de backups sem o backup procurado
    Quando restauro um backup desse mundo
    Então o resultado é Err(ApplicationError::NotFound { what: Backup })
    E não é mais Err(DomainError::InvalidBackupPath)

  @not-found @application
  Cenário: Criar backup de mundo inexistente retorna NotFound
    Dado um repositório de mundos sem o mundo procurado
    Quando crio um backup desse mundo
    Então o resultado é Err(ApplicationError::NotFound { what: World })

  @storage @application
  Cenário: Falha de cópia durante restore retorna Storage
    Dado um backup válido e um mundo válido
    E um diretório de destino que rejeita escrita
    Quando executo a cópia do backup para o mundo
    Então o resultado é Err(ApplicationError::Storage(_))
    E não é variant de validação

  @validation @application
  Cenário: Erro de validação de domínio propaga como Validation
    Dado um nome de pasta de mundo inválido
    Quando o use case valida a entrada
    Então o resultado é Err(ApplicationError::Validation(DomainError::InvalidWorldFolderName(_)))

  @storage @infrastructure
  Cenário: Falha de I/O no repositório retorna Storage
    Dado um diretório Users/ inacessível
    Quando o repositório lista mundos
    Então o resultado é Err(ApplicationError::Storage(_))

  @config @infrastructure
  Cenário: Falha ao ler config.toml retorna Config
    Dado um config.toml com conteúdo inválido
    Quando a configuração é carregada
    Então o resultado é Err(ApplicationError::Config(_))
    E não é DomainError::InvalidBackupPath

  @contrato @infrastructure
  Cenário: Bateria de contrato dos ports roda contra ApplicationError
    Dado os testes de contrato de WorldRepository e BackupRepository
    Quando executados contra as implementações reais (File*)
    Então o comportamento observado é idêntico ao dos fakes
    E os erros usam a taxonomia nova

  @ipc @tauri-adapter
  Cenário: CommandError serializa como objeto {code, message}
    Dado um command que retorna Err(ApplicationError::NotFound { what: Backup })
    Quando o erro cruza o IPC para o frontend
    Então o payload é um objeto com code "not_found"
    E o campo message contém o texto do erro interno

  @ipc @tauri-adapter
  Cenário: Erro fora da taxonomia cai em unknown
    Dado uma falha fora das categorias conhecidas
    Quando o command falha
    Então o code é "unknown"
    E a mensagem original acompanha o payload

  @extraction @frontend
  Cenário: getTauriError preserva objeto estruturado
    Dado um erro de invoke com code "storage" e message "permission denied"
    Quando chamo getTauriError com esse erro
    Então retorna o code e a message intactos

  @extraction @frontend
  Cenário: getTauriError não descarta payload desconhecido
    Dado um erro de invoke que é uma string crua
    Quando chamo getTauriError com esse erro
    Então retorna code "unknown" e a string como message

  @traducao @frontend
  Cenário: humanMessage mapeia código para mensagem PT-BR
    Dado um TauriError com code "storage"
    Quando chamo humanMessage com esse erro
    Então a mensagem retornada é o texto PT-BR da categoria storage

  @regressao @frontend
  Cenário: WorldList exibe a mensagem real em caso de falha
    Dado cmd_list_worlds rejeitando com um erro estruturado
    Quando a listagem de mundos falha
    Então a tela de erro exibe a mensagem mapeada da categoria recebida
    E não exibe "Erro desconhecido" fixo

  @error-boundary @frontend
  Cenário: ErrorBoundary captura crash de render
    Dado um componente filho que lança exceção ao renderizar
    Quando o React propaga o erro
    Então o ErrorBoundary renderiza a tela de falha com botão "Recarregar"
    E a página não fica em branco

  @settings @frontend
  Cenário: Settings exibe a causa real de falha de configuração
    Dado cmd_get_config rejeitando com code "config"
    Quando a página de configurações carrega
    Então o toast de erro contém a mensagem mapeada da categoria config

  @startup @integration
  Cenário: Startup sem diretório do Minecraft avisa antes de encerrar
    Dado o diretório do Minecraft Bedrock inexistente
    Quando o aplicativo inicializa
    Então uma message box nativa é exibida com o caminho esperado
    E o processo encerra com código diferente de zero
