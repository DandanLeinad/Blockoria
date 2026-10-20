import { useRouteError } from "react-router-dom";
import { getTauriError } from "../utils/tauriError";
import { ErrorScreen } from "./ErrorScreen";

/**
 * Router errorElement (spec 07, RF-06): React Router catches errors
 * thrown by route elements and loaders and renders this instead.
 */
export function RouteError() {
  const error = useRouteError();
  const detail = getTauriError(error).message;

  return (
    <ErrorScreen detail={detail === "Erro desconhecido" ? null : detail} />
  );
}
