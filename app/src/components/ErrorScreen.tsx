import { Button } from "./ui/button";
import { Icon } from "./ui/Icons";

interface ErrorScreenProps {
  detail?: string | null;
}

/**
 * Shared crash fallback (spec 07, RF-06): used by the global
 * ErrorBoundary and by the router errorElement. Never a white screen.
 */
export function ErrorScreen({ detail }: ErrorScreenProps) {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center gap-4 px-6 text-center">
      <div className="flex h-16 w-16 items-center justify-center rounded-full bg-destructive/10">
        <Icon name="alertTriangle" className="h-8 w-8 text-destructive" />
      </div>
      <div>
        <h1 className="text-lg font-semibold text-foreground">
          Algo deu errado
        </h1>
        <p className="mt-1 text-sm text-muted-foreground">
          Ocorreu um erro inesperado. Tente recarregar o aplicativo.
        </p>
        {detail && (
          <p className="mt-2 break-all font-mono text-xs text-muted-foreground/70">
            {detail}
          </p>
        )}
      </div>
      <Button onClick={() => window.location.reload()}>Recarregar</Button>
    </div>
  );
}
