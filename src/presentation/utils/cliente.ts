import type { Cliente } from "../../domain/entities";

export function clienteLabel(cliente: Cliente): string {
  const name = [cliente.nombre, cliente.apellido]
    .filter(Boolean)
    .join(" ");
  return name || cliente.telefono || cliente.email || "Sin datos";
}
