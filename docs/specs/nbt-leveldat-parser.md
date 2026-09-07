# Spec: LE-NBT Parser para level.dat (Minecraft Bedrock)

> **Status:** Em desenvolvimento
> **Branch:** `feat/nbt-leveldat-parser`
> **Relacionado:** `blockoria-infrastructure` crate

---

## 1. Contexto e Motivação

O arquivo `level.dat` do Minecraft Bedrock Edition armazena metadados globais do mundo (versão, seed, nome, modo de jogo, spawn, último acesso) em formato **Little-Endian NBT (LE-NBT)**.

Atualmente, `FileWorldRepository::parse_level_dat_version` (linha 195-200) retorna `None` (stub). Precisamos de um parser completo para extrair `WorldVersion` e, futuramente, outros metadados.

**Objetivo:** Implementar parser LE-NBT genérico, seguro e idiomático em Rust, integrado à arquitetura Clean Architecture existente.

---

## 2. Formato LE-NBT (Referência)

### 2.1 Header do level.dat (8 bytes)

| Offset | Tamanho | Tipo | Descrição |
|--------|---------|------|-----------|
| 0-3 | 4 bytes | `i32` LE | Versão/tipo do arquivo |
| 4-7 | 4 bytes | `i32` LE | Tamanho do payload NBT em bytes |

### 2.2 Payload NBT (após header)

Formato Little-Endian NBT (diferente do Java Edition que é Big-Endian).

**Tipos de Tag (IDs 0-12):**

| ID | Nome | Descrição |
|----|------|-----------|
| 0 | TAG_End | Fim de Compound/List |
| 1 | TAG_Byte | `i8` signed |
| 2 | TAG_Short | `i16` LE |
| 3 | TAG_Int | `i32` LE |
| 4 | TAG_Long | `i64` LE |
| 5 | TAG_Float | `f32` LE |
| 6 | TAG_Double | `f64` LE |
| 7 | TAG_Byte_Array | `[i32 length][bytes...]` |
| 8 | TAG_String | `[u16 length][UTF-8 bytes...]` |
| 9 | TAG_List | `[u8 subtype][i32 length][elements...]` |
| 10 | TAG_Compound | Repetido: `[u8 tag_id][string name][payload]` até TAG_End |
| 11 | TAG_Int_Array | `[i32 length][i32 values...]` |
| 12 | TAG_Long_Array | `[i32 length][i64 values...]` |

**Regras:**
- Todos os números: **Little-Endian**
- Strings: `u16` length + UTF-8
- TAG_List: todos elementos mesmo tipo (subtype)
- TAG_Compound: pares nome/valor até TAG_End (id=0)

---

## 3. Estrutura de Dados do level.dat (Bedrock)

Estrutura típica do root compound:

```text
TAG_Compound("")  // root sem nome ou "Data"
├── TAG_Int("formatVersion")           // ex: 123
├── TAG_Int("levelVersion")            // ex: 1
├── TAG_String("LevelName")            // "Meu Mundo"
├── TAG_Long("RandomSeed")             // seed do mundo
├── TAG_Int("GameType")                // 0=Survival, 1=Creative, 2=Adventure
├── TAG_Int("lastOpenedWithVersion")[5] // TAG_Int_Array ou TAG_List de TAG_Int
├── TAG_Long("LastPlayed")             // timestamp Unix ms
├── TAG_Int("SpawnX"), TAG_Int("SpawnY"), TAG_Int("SpawnZ")
└── ...outros campos
```

**Campo alvo inicial:** `lastOpenedWithVersion` → `WorldVersion [u16; 5]`

---

## 4. Arquitetura e Localização

### 4.1 Novo Módulo: `blockoria-infrastructure/src/nbt/`

```
src/nbt/
├── mod.rs              # Exports públicos
├── error.rs            # NbtError (thiserror)
├── tag_type.rs         # NbtTagType enum (repr u8, TryFrom<u8>)
├── value.rs            # NbtValue enum + NbtList wrapper
├── reader.rs           # LeReader<R: Read> (LE binary reading)
├── parser.rs           # Parser genérico (recursivo, depth tracking)
├── level_dat.rs        # LevelDatHeader, LevelDatParser, extract_version
└── json.rs             # Serialização JSON opcional (simples + tipado)
```

### 4.2 Integração Existente

**`FileWorldRepository::parse_level_dat_version`** (linha 195-200):
```rust
// ANTES (stub):
fn parse_level_dat_version(path: &Path) -> Option<WorldVersion> { None }

// DEPOIS:
fn parse_level_dat_version(path: &Path) -> Option<WorldVersion> {
    let file = fs::File::open(path).ok()?;
    let mut parser = LevelDatParser::new(file).ok()?;
    let nbt = parser.parse().ok()?;
    extract_world_version(&nbt)
}
```

**`blockoria-infrastructure/src/lib.rs`:**
```rust
pub mod nbt;
pub use nbt::{NbtValue, NbtError, LevelDatParser, extract_world_version};
```

---

## 5. Representação Interna (AST NBT)

### 5.1 NbtTagType

```rust
#[repr(u8)]
pub enum NbtTagType {
    End = 0, Byte = 1, Short = 2, Int = 3, Long = 4,
    Float = 5, Double = 6, ByteArray = 7, String = 8,
    List = 9, Compound = 10, IntArray = 11, LongArray = 12,
}
impl TryFrom<u8> for NbtTagType { ... }
```

### 5.2 NbtValue

```rust
pub enum NbtValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(NbtList),                    // Wrapper preserva element_type
    Compound(BTreeMap<String, NbtValue>),  // Ordem determinística
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

pub struct NbtList {
    pub element_type: NbtTagType,
    pub values: Vec<NbtValue>,
}
```

**Por que `BTreeMap`?** Ordem determinística → testes estáveis, JSON consistente, debug fácil. Overhead irrelevante para metadados pequenos.

**Por que `NbtList` wrapper?** TAG_List vazia perde `subtype` se for apenas `Vec<NbtValue>`. Wrapper preserva info para round-trip futuro.

---

## 6. Leitura Binária (LeReader)

```rust
pub struct LeReader<R: Read> {
    reader: BufReader<R>,
    offset: u64,
}

impl<R: Read> LeReader<R> {
    pub fn new(reader: R) -> Self { ... }
    pub fn offset(&self) -> u64 { self.offset }

    // LE reading via from_le_bytes (sem byteorder dep)
    pub fn read_i8(&mut self) -> Result<i8, NbtError>
    pub fn read_u8(&mut self) -> Result<u8, NbtError>
    pub fn read_i16(&mut self) -> Result<i16, NbtError>
    pub fn read_u16(&mut self) -> Result<u16, NbtError>
    pub fn read_i32(&mut self) -> Result<i32, NbtError>
    pub fn read_i64(&mut self) -> Result<i64, NbtError>
    pub fn read_f32(&mut self) -> Result<f32, NbtError>
    pub fn read_f64(&mut self) -> Result<f64, NbtError>
    pub fn read_string(&mut self) -> Result<String, NbtError>
}
```

---

## 7. Limites de Segurança (Hardcoded Constants)

```rust
const MAX_STRING_LENGTH: usize = 1_000_000;        // 1 MB
const MAX_ARRAY_LENGTH: usize = 10_000_000;        // 10M elementos (~40MB)
const MAX_LIST_LENGTH: usize = 1_000_000;          // 1M elementos
const MAX_COMPOUND_ENTRIES: usize = 100_000;       // 100K entradas
const MAX_NESTING_DEPTH: usize = 128;              // Stack overflow prevention
const MAX_LEVEL_DAT_SIZE: usize = 10_000_000;      // 10 MB arquivo inteiro
```

**Aplicados em:**
- `read_string`: valida length ≤ MAX_STRING_LENGTH
- Arrays/Lists: valida length ≤ MAX_ARRAY_LENGTH/MAX_LIST_LENGTH
- Compound: conta entradas, valida ≤ MAX_COMPOUND_ENTRIES
- Recursão: `depth` parameter, valida ≤ MAX_NESTING_DEPTH
- `LevelDatParser`: valida `nbt_size` ≤ MAX_LEVEL_DAT_SIZE, usa `take(nbt_size)` no reader

---

## 8. Tratamento de Erros (NbtError)

```rust
#[derive(Debug, thiserror::Error)]
pub enum NbtError {
    #[error("Unexpected EOF at offset {offset}")]
    UnexpectedEof { offset: u64 },

    #[error("Unknown tag ID {id} at offset {offset}")]
    UnknownTag { id: u8, offset: u64 },

    #[error("Invalid UTF-8 in string at offset {offset}: {source}")]
    InvalidUtf8 { offset: u64, source: std::str::Utf8Error },

    #[error("Negative length {len} for {context} at offset {offset}")]
    NegativeLength { len: i32, context: &'static str, offset: u64 },

    #[error("Excessive length {len} for {context} at offset {offset} (max {max})")]
    ExcessiveLength { len: i32, context: &'static str, max: usize, offset: u64 },

    #[error("Maximum nesting depth ({max}) exceeded at offset {offset}")]
    MaxDepthExceeded { max: usize, offset: u64 },

    #[error("IO error at offset {offset}: {source}")]
    Io { offset: u64, source: std::io::Error },

    #[error("Invalid level.dat header: {reason}")]
    InvalidHeader { reason: String },
}
```

**Princípios:**
- Zero `unwrap()`/`expect()` na lógica do parser
- Todos erros carregam `offset` para debug
- `NegativeLength` previne `vec![0; len as usize]` com len negativo
- `ExcessiveLength` previne OOM allocation

---

## 9. Extração de WorldVersion

```rust
// Em level_dat.rs
pub fn extract_world_version(nbt: &NbtValue) -> Option<WorldVersion> {
    // Navega: root -> "Data" (ou root direto) -> "lastOpenedWithVersion"
    // Aceita: TAG_Int_Array (id=11) OU TAG_List de TAG_Int (id=9, subtype=3)
    // Converte i32[] → [u16; 5] validando 5 elementos ≥ 0
}
```

---

## 10. Serialização JSON (Opcional)

Duas modalidades em `json.rs`:

```rust
// 1. Simples (para exibição)
pub fn to_json_simple(value: &NbtValue) -> serde_json::Value { ... }
// {"LevelName": "Mundo", "lastOpenedWithVersion": [1,21,0,0,0]}

// 2. Tipado (preserva info NBT completa)
pub fn to_json_typed(value: &NbtValue) -> serde_json::Value { ... }
// {"type": "Compound", "value": {"LevelName": {"type": "String", "value": "Mundo"}}}
```

**Feature gate:** Apenas com feature `"serde"` (consistente com domain).

---

## 11. Testes (TDD)

### 11.1 Unitários (inline `#[cfg(test)]`)

| Módulo | Testes-chave |
|--------|--------------|
| `tag_type` | `try_from_valid`, `try_from_invalid` |
| `value` | `construct_all`, `pattern_match`, `eq` |
| `reader` | `read_i32_le`, `read_string`, `eof`, `limits` |
| `parser` | Cada tag type, nested, empty list preserves type, max depth |
| `level_dat` | `parse_header`, `extract_version`, `invalid_size` |

### 11.2 Integração (`tests/nbt_integration_tests.rs`)

- `level.dat` sintético válido → version correta
- `level.dat` corrompido → erro apropriado
- Round-trip parse → JSON → value construction

### 11.3 Contrato (em `test_contract.rs`)

```rust
fn test_level_dat_version_parsing<R: WorldRepository>(repo: &R, ctx: &ContractTestContext)
```

---

## 12. Critérios de Aceite

- [ ] `cargo fmt` passa
- [ ] `cargo clippy -D warnings` passa
- [ ] **134 testes existentes** continuam passando
- [ ] **≥ 40 novos testes** NBT passando
- [ ] Zero `unwrap()`/`expect()` na lógica do parser
- [ ] `parse_level_dat_version` retorna `WorldVersion` correta
- [ ] Limites impedem OOM/stack overflow
- [ ] Rustdoc em todos os `pub` items

---

## 13. Fora de Escopo (Futuro)

- Parser completo LevelDB (`db/` - chunks, entities, block states)
- Cache de parsing
- CLI tool para debug
- Extração de mais campos: `LevelName`, `Seed`, `GameType`, `Spawn`, `LastPlayed`

---

## 14. Decisões Arquiteturais (ADR)

| Decisão | Escolha | Justificativa |
|---------|---------|---------------|
| `BTreeMap` vs `HashMap` | `BTreeMap` | Ordem determinística |
| List vazia type info | `NbtList` wrapper | Spec compliance |
| Dependências extras | **Zero** (std only) | Minimalismo |
| Serde | Feature gate `"serde"` | Consistência domain |
| JSON | Dois modos | Flexibilidade |

---

## 15. Próximos Passos (Pós-Spec)

1. **BDD:** Criar `tests/features/nbt_leveldat_parser.feature` com cenários Gherkin
2. **TDD:** Testes unitários → implementação por módulo (error → tag_type → value → reader → parser → level_dat → json)
3. **Integração:** Atualizar `FileWorldRepository` + testes de contrato
4. **Verificação:** `cargo test`, `fmt`, `clippy`, `deny`
5. **Documentação:** Rustdoc + atualizar docs se necessário
