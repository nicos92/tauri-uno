<script setup lang="ts">
import Toasts from "./presentation/components/Toasts.vue";
import ConfirmDialog from "./presentation/components/ConfirmDialog.vue";
</script>

<template>
  <router-view />
  <Toasts />
  <ConfirmDialog />
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

button,
input,
select,
textarea {
  font: inherit;
  color: inherit;
}

button,
select {
  appearance: none;
  -webkit-appearance: none;
}

select {
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'><path d='M1 1l5 5 5-5' fill='none' stroke='%236b7280' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'/></svg>");
  background-repeat: no-repeat;
  background-position: right 0.75rem center;
  padding-right: 2.5rem;
}

button:not(.btn-icon).btn-primary,
button:not(.btn-icon).btn-secondary,
button:not(.btn-icon).btn-danger {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  line-height: 1.2;
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
  color-scheme: light;
  --color-bg: #f3f4f6;
  --color-surface: #ffffff;
  --color-surface-2: #f8fafc;
  --color-text: #111827;
  --color-text-muted: #6b7280;
  --color-border: #e5e7eb;
  --color-primary: #3f2281;
  --color-secondary: var(--color-secondary);
  --color-danger: var(--color-danger);
}

html[data-theme="dark"] {
  color-scheme: dark;
  --color-bg: #020617;
  --color-surface: #0f172a;
  --color-surface-2: #111c33;
  --color-text: #e5e7eb;
  --color-text-muted: #94a3b8;
  --color-border: #1f2937;
  --color-primary: #3f2281;
  --color-secondary: var(--color-secondary);
  --color-danger: var(--color-danger);
}

.print-area {
  display: none;
}

@media print {
  body > * {
    display: none !important;
  }

  body > .print-area {
    display: block !important;
  }

  .print-area * {
    display: revert !important;
  }

  .print-area {
    padding: 1rem;
  }

  .print-summary {
    border: 1px solid #000;
    padding: 0.75rem 1rem;
    margin: 0.75rem 0;
  }

  .print-area table {
    width: 100%;
    border-collapse: collapse;
  }

  .print-area th,
  .print-area td {
    border: 1px solid #000;
    padding: 0.5rem;
    text-align: left;
  }

  .print-total {
    font-weight: bold;
    text-align: right;
  }

  .print-line {
    text-align: right;
    margin: 0.25rem 0;
  }

  .print-obs {
    margin-top: 1rem;
  }
}
</style>
