# encoding: utf-8
# language: pt

@feature-flags
Funcionalidade: Mecanismo de feature flags
  Como desenvolvedor do Blockoria
  Eu quero flags de lançamento com default no código e sobreposição só em desenvolvimento
  Para que trabalho incompleto seja integrado à main escondido, sem alcançar o usuário final

  Contexto:
    Port FeatureFlags em application; impl CodeFeatureFlags em infrastructure
    Defaults moram no enum Flag (default_enabled); domain não sabe que flags existem
    Em debug, BLOCKORIA_FLAGS="flag_a,flag_b" liga flags por nome (snake_case)
    Em release o código do env nem é compilado (#[cfg(debug_assertions)])
    O front é consumidor read-only via cmd_feature_flags

  @default @infrastructure
  Cenário: Flag sem sobreposição resolve para o default do código
    Dado uma flag cujo default_enabled é false
    E nenhuma variável de ambiente de desenvolvimento (release, ou debug sem BLOCKORIA_FLAGS)
    Quando a flag é resolvida
    Então o resultado é false

  @default @infrastructure
  Cenário: Default não-falso do código é respeitado quando não há sobreposição
    Dado uma flag cujo default_enabled é true
    E nenhuma variável de ambiente de desenvolvimento
    Quando a flag é resolvida
    Então o resultado é true

  @env-var @infrastructure
  Cenário: Env var de debug com o nome da flag liga a flag
    Dado BLOCKORIA_FLAGS contendo "minha_flag"
    Quando a flag "minha_flag" é resolvida
    Então o resultado é true

  @env-var @infrastructure
  Cenário: Lista com múltiplos tokens separados por vírgula e espaços
    Dado BLOCKORIA_FLAGS contendo "outra_flag, minha_flag ,terceira_flag"
    Quando a flag "minha_flag" é resolvida
    Então o resultado é true

  @env-var @infrastructure
  Cenário: Token que não casa com nenhuma flag não liga nada
    Dado BLOCKORIA_FLAGS contendo "outra_flag,flag_inexistente"
    Quando a flag "minha_flag" é resolvida
    Então o resultado é o default (false)

  @env-var @infrastructure
  Cenário: Lista vazia ou só de espaços não liga nada
    Dado BLOCKORIA_FLAGS contendo " , "
    Quando a flag "minha_flag" é resolvida
    Então o resultado é o default (false)

  @ipc @tauri-adapter
  Cenário: Command expõe o mapa resolvido das flags conhecidas
    Dado o estado da aplicação com as flags padrão
    Quando o front invoca cmd_feature_flags
    Então recebe uma entrada por flag em Flag::ALL com o valor resolvido
    E hoje, sem nenhuma flag registrada (spec 08), o mapa está vazio
