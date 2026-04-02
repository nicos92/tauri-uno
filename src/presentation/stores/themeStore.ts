import { defineStore } from "pinia";
import { ref, watchEffect, onMounted } from "vue";

export type ThemeMode = "light" | "dark" | "system";

const THEME_STORAGE_KEY = "app-theme";

function getSystemPrefersDark(): boolean {
  if (typeof window === "undefined" || !window.matchMedia) {
    return false;
  }

  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

function applyThemeClass(mode: ThemeMode) {
  if (typeof document === "undefined") return;

  const root = document.documentElement;
  const effective = mode === "system" ? (getSystemPrefersDark() ? "dark" : "light") : mode;

  root.dataset.theme = effective;
}

export const useThemeStore = defineStore("theme", () => {
  const mode = ref<ThemeMode>("system");

  function loadFromStorage() {
    const stored = localStorage.getItem(THEME_STORAGE_KEY) as ThemeMode | null;
    if (stored === "light" || stored === "dark" || stored === "system") {
      mode.value = stored;
    } else {
      mode.value = "system";
    }
    applyThemeClass(mode.value);
  }

  function setMode(newMode: ThemeMode) {
    mode.value = newMode;
    localStorage.setItem(THEME_STORAGE_KEY, newMode);
    applyThemeClass(newMode);
  }

  if (typeof window !== "undefined") {
    onMounted(() => {
      loadFromStorage();

      if (window.matchMedia) {
        const media = window.matchMedia("(prefers-color-scheme: dark)");
        const handler = () => {
          if (mode.value === "system") {
            applyThemeClass("system");
          }
        };
        media.addEventListener("change", handler);
      }
    });
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

