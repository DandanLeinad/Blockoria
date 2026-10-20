export type ErrorCode =
  "not_found" | "validation" | "storage" | "config" | "unknown";

export interface TauriError {
  code: ErrorCode;
  message: string;
}

const ERROR_CODES: readonly string[] = [
  "not_found",
  "validation",
  "storage",
  "config",
  "unknown",
];

function isErrorCode(value: unknown): value is ErrorCode {
  return typeof value === "string" && ERROR_CODES.includes(value);
}

/**
 * Extracts the structured IPC error (`{ code, message }`) from anything
 * `invoke()` can reject with (spec 07, RF-04). Never discards the payload:
 * unknown shapes fall back to `unknown` carrying the raw content.
 */
export function getTauriError(error: unknown): TauriError {
  if (typeof error === "object" && error !== null && "message" in error) {
    const { code, message } = error as { code: unknown; message: unknown };
    if (typeof message === "string") {
      return { code: isErrorCode(code) ? code : "unknown", message };
    }
  }
  if (typeof error === "string" && error.length > 0) {
    return { code: "unknown", message: error };
  }
  return { code: "unknown", message: "Erro desconhecido" };
}

const HUMAN_MESSAGES: Record<ErrorCode, string> = {
  not_found: "Mundo ou backup não encontrado",
  validation: "Dados inválidos",
  storage: "Erro ao acessar arquivos (permissões ou disco)",
  config: "Erro nas configurações",
  unknown: "Erro desconhecido",
};

/**
 * Maps an error code to the human PT-BR message (spec 07, RF-04).
 * The raw technical message is kept in `TauriError.message` for detail views.
 */
export function humanMessage(error: TauriError): string {
  return HUMAN_MESSAGES[error.code];
}
