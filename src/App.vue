<script setup lang="ts">
</script>

<template>
  <router-view />
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100%;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  background-color: var(--color-bg);
  color: var(--color-text);
}

body {
  transition: background-color 0.2s ease, color 0.2s ease;
}

html[data-theme="light"] {
  --color-bg: #f3f4f6;
  --color-surface: #ffffff;
  --color-surface-2: #f8fafc;
  --color-text: #111827;
  --color-text-muted: #6b7280;
  --color-border: #e5e7eb;
}

html[data-theme="dark"] {
  --color-bg: #020617;
  --color-surface: #0f172a;
  --color-surface-2: #111c33;
  --color-text: #e5e7eb;
  --color-text-muted: #94a3b8;
  --color-border: #1f2937;
}
</style>
