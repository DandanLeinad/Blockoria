# encoding: utf-8
# language: pt

@nbt @infrastructure
Funcionalidade: Parser LE-NBT para level.dat do Minecraft Bedrock
  Como desenvolvedor do Blockoria
  Eu quero parsear corretamente o arquivo level.dat
  Para extrair metadados do mundo (versão, seed, nome, etc.)

  Contexto:
    O Minecraft Bedrock usa Little-Endian NBT (LE-NBT) no level.dat
    Header: 8 bytes (i32 version, i32 nbt_size)
    Payload: NBT tags (IDs 0-12) little-endian
    Campo alvo inicial: lastOpenedWithVersion -> WorldVersion [u16; 5]

@tag_type @domain
  Cenário: Converter u8 válido para NbtTagType
    Dado um byte representando TAG_Int (valor 3)
    Quando converto usando TryFrom<u8>
    Então o resultado é Ok(NbtTagType::Int)

  Cenário: Rejeitar u8 inválido para NbtTagType
    Dado um byte com valor 99 (não existe na spec)
    Quando converto usando TryFrom<u8>
    Então o resultado é Err

@value @domain
  Cenário: Construir todos os variantes de NbtValue
    Dado os construtores de NbtValue
    Quando crio cada variante com dados válidos
    Então todos são construídos sem erro
    E pattern matching funciona para cada variante

  Cenário: NbtList preserva element_type em lista vazia
    Dado uma TAG_List com subtype=3 (Int) e length=0
    Quando construo NbtList { element_type: Int, values: [] }
    Então element_type permanece Int
    E values está vazio

  Cenário: Compound usa BTreeMap para ordem determinística
    Dado um Compound com chaves "z", "a", "m"
    Quando itero sobre as entradas
    Então a ordem é "a", "m", "z" (alfabética)

@reader @infrastructure
  Cenário: Ler i32 little-endian corretamente
    Dado um buffer [0x78, 0x56, 0x34, 0x12] (LE para 0x12345678)
    Quando chamo read_i32()
    Então retorna Ok(305419896)

  Cenário: Ler string UTF-8 com length prefix
    Dado buffer [0x05, 0x00, 'H', 'e', 'l', 'l', 'o'] (u16=5 + bytes)
    Quando chamo read_string()
    Então retorna Ok("Hello")

  Cenário: EOF inesperado durante leitura
    Dado um reader com apenas 2 bytes disponíveis
    Quando chamo read_i32() (precisa 4 bytes)
    Então retorna Err(NbtError::UnexpectedEof)

  Cenário: Rejeitar string com length excessivo
    Dado buffer com u16 length = 2_000_000 (> MAX_STRING_LENGTH)
    Quando chamo read_string()
    Então retorna Err(NbtError::ExcessiveLength)

  Cenário: Rejeitar length negativo em array
    Dado buffer com i32 length = -100
    Quando chamo read_array_length()
    Então retorna Err(NbtError::NegativeLength)

@parser @infrastructure
  Cenário: Parse TAG_Byte
    Dado payload com tag_id=1 seguido de byte 42
    Quando chamo parse_payload(1)
    Então retorna NbtValue::Byte(42)

  Cenário: Parse TAG_Short
    Dado payload com tag_id=2 seguido de i16 LE 300
    Quando chamo parse_payload(2)
    Então retorna NbtValue::Short(300)

  Cenário: Parse TAG_Int
    Dado payload com tag_id=3 seguido de i32 LE 123456
    Quando chamo parse_payload(3)
    Então retorna NbtValue::Int(123456)

  Cenário: Parse TAG_Long
    Dado payload com tag_id=4 seguido de i64 LE 9999999999
    Quando chamo parse_payload(4)
    Então retorna NbtValue::Long(9999999999)

  Cenário: Parse TAG_Float
    Dado payload com tag_id=5 seguido de f32 LE 3.14
    Quando chamo parse_payload(5)
    Então retorna NbtValue::Float(3.14)

  Cenário: Parse TAG_Double
    Dado payload com tag_id=6 seguido de f64 LE 2.71828
    Quando chamo parse_payload(6)
    Então retorna NbtValue::Double(2.71828)

  Cenário: Parse TAG_Byte_Array
    Dado payload com tag_id=7, length=3, bytes [1,2,3]
    Quando chamo parse_payload(7)
    Então retorna NbtValue::ByteArray(vec![1,2,3])

  Cenário: Parse TAG_String
    Dado payload com tag_id=8, string "Mundo Teste"
    Quando chamo parse_payload(8)
    Então retorna NbtValue::String("Mundo Teste")

  Cenário: Parse TAG_List de Int não vazia
    Dado payload com tag_id=9, subtype=3 (Int), length=3, valores [1,2,3]
    Quando chamo parse_payload(9)
    Então retorna NbtList { element_type: Int, values: [Int(1), Int(2), Int(3)] }

  Cenário: Parse TAG_List vazia preserva element_type
    Dado payload com tag_id=9, subtype=4 (Long), length=0
    Quando chamo parse_payload(9)
    Então retorna NbtList { element_type: Long, values: [] }

  Cenário: Parse TAG_Compound simples
    Dado payload com tag_id=10, uma entrada "version"=Int(1), TAG_End
    Quando chamo parse_payload(10)
    Então retorna Compound com entrada "version" = Int(1)

  Cenário: Parse TAG_Compound aninhado
    Dado payload Compound contendo outro Compound "Data" com "version"=Int(2)
    Quando chamo parse_payload(10)
    Então retorna Compound aninhado corretamente

  Cenário: Parse TAG_Int_Array
    Dado payload com tag_id=11, length=5, valores [1,21,0,0,0]
    Quando chamo parse_payload(11)
    Então retorna NbtValue::IntArray(vec![1,21,0,0,0])

  Cenário: Parse TAG_Long_Array
    Dado payload com tag_id=12, length=2, valores [100, 200]
    Quando chamo parse_payload(12)
    Então retorna NbtValue::LongArray(vec![100, 200])

  Cenário: Detectar TAG_End encerra Compound
    Dado parser dentro de parse_compound
    Quando lê tag_id=0
    Então retorna o Compound acumulado (não erro)

  Cenário: Rejeitar tag_id desconhecido
    Dado payload com tag_id=99
    Quando chamo parse_payload(99)
    Então retorna Err(NbtError::UnknownTag)

  Cenário: Respeitar limite de profundidade de recursão
    Dado Compound aninhado 200 níveis (> MAX_NESTING_DEPTH=128)
    Quando chamo parse_payload(10)
    Então retorna Err(NbtError::MaxDepthExceeded)

  Cenário: Respeitar limite de entradas em Compound
    Dado Compound com 150_000 entradas (> MAX_COMPOUND_ENTRIES)
    Quando chamo parse_compound()
    Então retorna Err(NbtError::ExcessiveLength)

  Cenário: Respeitar limite de elementos em List
    Dado List com 2_000_000 elementos (> MAX_LIST_LENGTH)
    Quando chamo parse_payload(9)
    Então retorna Err(NbtError::ExcessiveLength)

@level_dat @infrastructure
  Cenário: Parse header válido do level.dat
    Dado arquivo com 8 bytes: version=123 (LE), nbt_size=456 (LE)
    Quando LevelDatParser lê o header
    Então retorna LevelDatHeader { version: 123, nbt_size: 456 }

  Cenário: Rejeitar header com nbt_size negativo
    Dado arquivo com nbt_size = -100
    Quando LevelDatParser valida header
    Então retorna Err(NbtError::InvalidHeader)

  Cenário: Rejeitar header com nbt_size excessivo
    Dado arquivo com nbt_size = 20_000_000 (> MAX_LEVEL_DAT_SIZE)
    Quando LevelDatParser valida header
    Então retorna Err(NbtError::InvalidHeader)

  Cenário: Parse completo level.dat sintético válido
    Dado level.dat com header válido + NBT contendo lastOpenedWithVersion=[1,21,0,0,0]
    Quando LevelDatParser.parse()
    Então retorna NbtValue::Compound com estrutura correta

  Cenário: Extrair WorldVersion de TAG_Int_Array
    Dado NBT com lastOpenedWithVersion como TAG_Int_Array [1,21,0,0,0]
    Quando chamo extract_world_version()
    Então retorna Some(WorldVersion([1,21,0,0,0]))

  Cenário: Extrair WorldVersion de TAG_List de TAG_Int
    Dado NBT com lastOpenedWithVersion como TAG_List(subtype=Int, length=5, [1,21,0,0,0])
    Quando chamo extract_world_version()
    Então retorna Some(WorldVersion([1,21,0,0,0]))

  Cenário: Retornar None quando lastOpenedWithVersion ausente
    Dado NBT sem campo lastOpenedWithVersion
    Quando chamo extract_world_version()
    Então retorna None

  Cenário: Retornar None quando lastOpenedWithVersion tem formato inválido
    Dado NBT com lastOpenedWithVersion como String "errado"
    Quando chamo extract_world_version()
    Então retorna None

  Cenário: Validar que parser não lê além do nbt_size declarado
    Dado level.dat com nbt_size=100 mas arquivo tem 500 bytes
    Quando LevelDatParser.parse()
    Então lê exatamente 100 bytes do payload

@json @infrastructure
  Cenário: Serialização JSON simples
    Dado NbtValue::String("Teste")
    Quando chamo to_json_simple()
    Então retorna JSON {"type": "String", "value": "Teste"} ou equivalente simples

  Cenário: Serialização JSON tipado
    Dado NbtValue::Int(42)
    Quando chamo to_json_typed()
    Então retorna JSON preservando tipo: {"type": "Int", "value": 42}

  Cenário: JSON simples de Compound
    Dado Compound {"nome": String("Mundo"), "versao": Int(1)}
    Quando chamo to_json_simple()
    Então retorna {"nome": "Mundo", "versao": 1}

@integration @infrastructure
  Cenário: Parse level.dat real do Minecraft Bedrock
    Dado arquivo level.dat real de um mundo Bedrock
    Quando LevelDatParser.parse() e extract_world_version()
    Então retorna WorldVersion válida (ex: [1,21,0,0,0])

  Cenário: Arquivo corrompido retorna erro apropriado
    Dado level.dat truncado no meio
    Quando LevelDatParser.parse()
    Então retorna NbtError (não panic)

@contract @application
  Cenário: FileWorldRepository usa parser para extrair versão
    Dado FileWorldRepository com mundo contendo level.dat válido
    Quando list_all() ou find_by_folder_name()
    Então World.version() vem do parser (não Default)

---

# Tags para execução seletiva:
# @domain      - Lógica pura (tag_type, value) - sem I/O
# @infrastructure - I/O real (reader, parser, level_dat, json)
# @integration - Arquivo real level.dat
# @contract    - Testes de contrato via WorldRepository port