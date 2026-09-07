# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 DandanLeinad

#!/usr/bin/env python3
"""Script para adicionar header SPDX nos arquivos do projeto Blockoria.

Suporta:
- Rust (.rs) - comentários //
- TypeScript/JavaScript (.ts, .tsx, .js, .jsx) - comentários //
- CSS (.css) - comentários /* */
- HTML (.html) - comentários <!-- -->

Uso:

    python scripts/add_license_header.py

    python scripts/add_license_header.py crates/blockoria-domain

    python scripts/add_license_header.py crates/blockoria-domain/src/lib.rs
"""

import logging
import re
import sys
from pathlib import Path

logging.basicConfig(
    level=logging.INFO,
    format="%(levelname)s: %(message)s",
)

logger = logging.getLogger(__name__)

SUPPORTED_EXTENSIONS = {
    ".rs",
    ".ts",
    ".tsx",
    ".js",
    ".jsx",
    ".css",
    ".html",
}

# Mapeia extensão para formato de header SPDX
SPDX_HEADERS = {
    ".rs": """// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad""",
    ".ts": """// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad""",
    ".tsx": """// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad""",
    ".js": """// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad""",
    ".jsx": """// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad""",
    ".css": """/* SPDX-License-Identifier: AGPL-3.0-or-later
 * Copyright (C) 2026 DandanLeinad */""",
    ".html": """<!-- SPDX-License-Identifier: AGPL-3.0-or-later
     Copyright (C) 2026 DandanLeinad -->""",
}

# Prefixos para detecção
SPDX_PREFIXES = {
    ".rs": "// SPDX-License-Identifier:",
    ".ts": "// SPDX-License-Identifier:",
    ".tsx": "// SPDX-License-Identifier:",
    ".js": "// SPDX-License-Identifier:",
    ".jsx": "// SPDX-License-Identifier:",
    ".css": "/* SPDX-License-Identifier:",
    ".html": "<!-- SPDX-License-Identifier:",
}

OLD_AGPL_HEADER_PATTERN = re.compile(
    r"// blockoria\s*\n"
    r"// Copyright \(C\) 2026\s+DandanLeinad\s*\n"
    r"(?://.*\n)*?"
    r"// You should have received a copy of the GNU Affero General Public License\s*\n"
    r"// along with this program\. If not, see <https://www\.gnu\.org/licenses/>",
    re.MULTILINE,
)

IGNORE_DIRS = {
    ".git",
    "target",
    ".venv",
    "node_modules",
    "site",
}


def get_spdx_header(ext: str) -> str:
    """Retorna o header SPDX apropriado para a extensão."""
    return SPDX_HEADERS.get(ext, SPDX_HEADERS[".rs"])


def get_spdx_prefix(ext: str) -> str:
    """Retorna o prefixo esperado para detecção."""
    return SPDX_PREFIXES.get(ext, "// SPDX-License-Identifier:")


def has_spdx_header(content: str, ext: str) -> bool:
    """Verifica se arquivo já possui SPDX."""

    prefix = get_spdx_prefix(ext)
    return content.lstrip().startswith(prefix)


def has_old_agpl_header(content: str) -> bool:
    """Verifica header antigo AGPL (formato Rust //)."""

    return bool(OLD_AGPL_HEADER_PATTERN.search(content))


def add_license_header(file_path: Path) -> bool:
    """Adiciona ou atualiza header SPDX."""

    try:
        ext = file_path.suffix
        content = file_path.read_text(encoding="utf-8")

        if has_spdx_header(content, ext):
            logger.info(f"[=] {file_path} - já possui SPDX")

            return False

        if has_old_agpl_header(content):
            new_content = OLD_AGPL_HEADER_PATTERN.sub(
                get_spdx_header(ext),
                content,
                count=1,
            )

            file_path.write_text(
                new_content,
                encoding="utf-8",
            )

            logger.info(f"[~] {file_path} - header antigo atualizado")

            return True

        new_content = f"{get_spdx_header(ext)}\n\n{content}"

        file_path.write_text(
            new_content,
            encoding="utf-8",
        )

        logger.info(f"[+] {file_path} - SPDX adicionado")

        return True

    except Exception as exc:
        logger.error(f"[-] {file_path} - erro: {exc}")

        return False


def find_source_files(path: Path | str) -> list[Path]:
    """Encontra arquivos suportados."""

    path = Path(path)

    if path.is_file():
        return [path] if path.suffix in SUPPORTED_EXTENSIONS else []

    source_files = []

    for ext in SUPPORTED_EXTENSIONS:
        for source_file in path.rglob(f"*{ext}"):
            if any(skip in source_file.parts for skip in IGNORE_DIRS):
                continue

            source_files.append(source_file)

    return sorted(source_files)


def main():

    target = sys.argv[1] if len(sys.argv) > 1 else "."

    target_path = Path(target)

    if not target_path.exists():
        logger.error(f"Caminho não existe: {target}")

        sys.exit(1)

    logger.info(f"Procurando arquivos em: {target_path.resolve()}")

    source_files = find_source_files(target_path)

    if not source_files:
        logger.warning("Nenhum arquivo suportado encontrado")

        sys.exit(0)

    added = 0

    for file_path in source_files:
        if add_license_header(file_path):
            added += 1

    logger.info(
        f"Resumo: {added} atualizado(s), "
        f"{len(source_files) - added} já estavam corretos"
    )


if __name__ == "__main__":
    main()
