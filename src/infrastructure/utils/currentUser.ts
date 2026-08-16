export function getCurrentUserId(): number {
  const stored = sessionStorage.getItem("currentUser");
  if (stored) {
    const user = JSON.parse(stored) as { id: number };
    return user.id;
  }
  return 0;
}
