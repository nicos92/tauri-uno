import { defineStore } from "pinia";
import { ref, watchEffect } from "vue";

export type ThemeMode = "light" | "dark" | "system";

const THEME_STORAGE_KEY = "app-theme";

function getSystemPrefersDark(): boolean {
  if (typeof window === "undefined" || !window.matchMedia) {
    return false;
  }

  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

function readStoredMode(): ThemeMode {
  try {
    const stored = localStorage.getItem(THEME_STORAGE_KEY);
    if (stored === "light" || stored === "dark" || stored === "system") {
      return stored;
    }
  } catch {
    // localStorage no disponible
  }
  return "system";
}

function applyThemeClass(mode: ThemeMode) {
  if (typeof document === "undefined") return;

  const root = document.documentElement;
  const effective = mode === "system" ? (getSystemPrefersDark() ? "dark" : "light") : mode;

  root.dataset.theme = effective;
}

export const useThemeStore = defineStore("theme", () => {
  const mode = ref<ThemeMode>(readStoredMode());

  function loadFromStorage() {
    mode.value = readStoredMode();
    applyThemeClass(mode.value);
  }

  function setMode(newMode: ThemeMode) {
    mode.value = newMode;
    try {
      localStorage.setItem(THEME_STORAGE_KEY, newMode);
    } catch {
      // localStorage no disponible
    }
    applyThemeClass(newMode);
  }

  if (typeof window !== "undefined" && window.matchMedia) {
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const handler = () => {
      if (mode.value === "system") {
        applyThemeClass("system");
      }
    };
    media.addEventListener("change", handler);
  }

  watchEffect(() => {
    applyThemeClass(mode.value);
  });

  return {
    mode,
    setMode,
    loadFromStorage,
  };
});
