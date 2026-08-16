export function todayLocal(): string {
  const now = new Date();
  const y = now.getFullYear();
  const m = String(now.getMonth() + 1).padStart(2, "0");
  const d = String(now.getDate()).padStart(2, "0");
  return `${y}-${m}-${d}`;
}

export function formatTimestamp(timestamp: string): string {
  const date = new Date(timestamp.replace(" ", "T") + "Z");
  return isNaN(date.getTime()) ? timestamp : date.toLocaleString();
}

export function dayStartToISO(date: string): string {
  return `${date}T00:00:00.000Z`;
}

export function dayEndToISO(date: string): string {
  return `${date}T23:59:59.999Z`;
}
