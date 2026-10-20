import { render, screen } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { ErrorBoundary } from "./ErrorBoundary";

function Thrower(): never {
  throw new Error("kaboom");
}

describe("ErrorBoundary (spec 07, RF-06)", () => {
  it("given a child that throws when rendering then fallback with reload button appears", () => {
    // Given
    const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});

    // When
    render(
      <ErrorBoundary>
        <Thrower />
      </ErrorBoundary>,
    );

    // Then
    expect(screen.getByText("Algo deu errado")).toBeInTheDocument();
    expect(screen.getByText("Recarregar")).toBeInTheDocument();
    expect(screen.queryByText("kaboom")).not.toBeInTheDocument();
    consoleSpy.mockRestore();
  });

  it("given healthy children when rendering then children pass through untouched", () => {
    // Given
    const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});

    // When
    render(
      <ErrorBoundary>
        <div>conteúdo são</div>
      </ErrorBoundary>,
    );

    // Then
    expect(screen.getByText("conteúdo são")).toBeInTheDocument();
    expect(screen.queryByText("Algo deu errado")).not.toBeInTheDocument();
    consoleSpy.mockRestore();
  });
});
