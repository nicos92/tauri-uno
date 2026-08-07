import type { ApiError } from "../../domain/entities";

function isApiError(e: unknown): e is ApiError {
  return (
    typeof e === "object" &&
    e !== null &&
    "message" in e &&
    typeof (e as ApiError).message === "string"
  );
}

export function toErrorMessage(e: unknown): string {
  if (isApiError(e)) {
    return e.message;
  }

  if (typeof e === "string") {
    return e;
  }

  if (e instanceof Error) {
    return e.message;
  }

  return "Ocurrió un error inesperado. Intente nuevamente.";
}
