import { describe, it, expect } from "vitest";
import { getTauriError, humanMessage } from "./tauriError";

describe("getTauriError", () => {
  it("given a structured IPC error when extracted then code and message are preserved", () => {
    // Given
    const ipcError = { code: "storage", message: "permission denied" };

    // When
    const result = getTauriError(ipcError);

    // Then
    expect(result.code).toBe("storage");
    expect(result.message).toBe("permission denied");
  });

  it("given an unknown code when extracted then falls back to unknown without losing the message", () => {
    // Given
    const ipcError = { code: "weird-code", message: "something" };

    // When
    const result = getTauriError(ipcError);

    // Then
    expect(result.code).toBe("unknown");
    expect(result.message).toBe("something");
  });

  it("given a raw string rejection when extracted then never discards the payload", () => {
    // Given
    const rawError = "boom";

    // When
    const result = getTauriError(rawError);

    // Then
    expect(result.code).toBe("unknown");
    expect(result.message).toBe("boom");
  });

  it("given an object with only a message when extracted then keeps it as unknown", () => {
    // Given
    const objectError = { message: "only message" };

    // When
    const result = getTauriError(objectError);

    // Then
    expect(result.code).toBe("unknown");
    expect(result.message).toBe("only message");
  });

  it("given an unrecognizable payload when extracted then generic unknown", () => {
    // Given
    const weirdError = 42;

    // When
    const result = getTauriError(weirdError);

    // Then
    expect(result.code).toBe("unknown");
    expect(result.message).toBe("Erro desconhecido");
  });
});

describe("humanMessage", () => {
  it("given not_found when translated then pt-br human text", () => {
    // Given
    const error = { code: "not_found" as const, message: "not found: backup" };

    // When
    const result = humanMessage(error);

    // Then
    expect(result).toBe("Mundo ou backup não encontrado");
  });

  it("given validation when translated then pt-br human text", () => {
    // Given
    const error = {
      code: "validation" as const,
      message: "validation: bad input",
    };

    // When
    const result = humanMessage(error);

    // Then
    expect(result).toBe("Dados inválidos");
  });

  it("given storage when translated then pt-br human text", () => {
    // Given
    const error = {
      code: "storage" as const,
      message: "storage: permission denied",
    };

    // When
    const result = humanMessage(error);

    // Then
    expect(result).toBe("Erro ao acessar arquivos (permissões ou disco)");
  });

  it("given config when translated then pt-br human text", () => {
    // Given
    const error = {
      code: "config" as const,
      message: "config: failed to read",
    };

    // When
    const result = humanMessage(error);

    // Then
    expect(result).toBe("Erro nas configurações");
  });

  it("given unknown when translated then generic pt-br text", () => {
    // Given
    const error = { code: "unknown" as const, message: "anything" };

    // When
    const result = humanMessage(error);

    // Then
    expect(result).toBe("Erro desconhecido");
  });
});
