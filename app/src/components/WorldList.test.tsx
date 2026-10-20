import { render, screen, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { WorldList } from "./WorldList";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("WorldList error surfacing (spec 07, RF-05)", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("given a structured rejection when loading fails then shows the mapped human message", async () => {
    // Given
    vi.mocked(invoke).mockRejectedValue({
      code: "storage",
      message: "permission denied",
    });

    // When
    render(<WorldList />);

    // Then
    await waitFor(() => {
      expect(screen.getByText("Erro ao carregar mundos")).toBeInTheDocument();
    });
    expect(
      screen.getByText("Erro ao acessar arquivos (permissões ou disco)"),
    ).toBeInTheDocument();
  });

  it("given a raw string rejection when loading fails then never discards the message", async () => {
    // Given
    vi.mocked(invoke).mockRejectedValue("boom");

    // When
    render(<WorldList />);

    // Then
    await waitFor(() => {
      expect(screen.getByText("Erro ao carregar mundos")).toBeInTheDocument();
    });
    expect(screen.getByText("boom")).toBeInTheDocument();
  });
});
