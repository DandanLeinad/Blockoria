# language: pt
Funcionalidade: Listar mundos do Minecraft Bedrock

  Contexto: O Blockoria deve mostrar todos os mundos detectados nos diretórios
  padrão do Minecraft Bedrock Edition no Windows (%APPDATA%\Minecraft Bedrock\).

  Cenário: Lista de mundos carregada com sucesso
    Dado que o Minecraft Bedrock tem mundos instalados
    E o app é iniciado
    Quando a tela principal carrega
    Então a lista de mundos deve ser exibida
    E cada mundo deve mostrar: nome do nível, versão, tipo (compartilhado/conta)
    E mundos com ícone devem exibir a imagem

  Cenário: Nenhum mundo encontrado
    Dado que o diretório de mundos está vazio
    E o app é iniciado
    Quando a tela principal carrega
    Então deve exibir estado vazio com mensagem "Nenhum mundo encontrado"
    E deve mostrar orientação de onde procurar mundos

  Cenário: Erro ao ler diretório de mundos
    Dado que o app não tem permissão para acessar o diretório
    E o app é iniciado
    Quando a tela principal tenta carregar
    Então deve exibir mensagem de erro amigável
    E deve mostrar botão "Tentar novamente"
    E ao clicar "Tentar novamente" deve refazer a requisição

  Cenário: Mundos de conta Xbox e Shared aparecem corretamente
    Dado que existem mundos em %APPDATA%\Minecraft Bedrock\Users\<user_id>\games\com.mojang\minecraftWorlds\
    E existem mundos em %APPDATA%\Minecraft Bedrock\Users\Shared\games\com.mojang\minecraftWorlds\
    Quando a lista carrega
    Então mundos Shared devem mostrar badge "Compartilhado"
    E mundos de conta devem mostrar ID da conta (truncado)
    E ambos os tipos devem aparecer na mesma lista

  Cenário: Navegação por teclado na lista
    Dado que a lista de mundos está visível
    Quando o usuário pressiona Tab
    Então o foco deve mover entre os cards de mundo
    E Enter deve ser aceito (para futura ação de backup)
