import { ref } from "vue";

export interface ConfirmOptions {
  title?: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  variant?: "danger" | "primary";
}

interface PendingConfirm {
  options: ConfirmOptions;
  resolve: (value: boolean) => void;
}

const active = ref<PendingConfirm | null>(null);

function confirm(options: ConfirmOptions): Promise<boolean> {
  if (active.value) {
    return Promise.resolve(false);
  }

  return new Promise<boolean>((resolve) => {
    active.value = {
      options: {
        title: "Confirmar",
        confirmText: "Confirmar",
        cancelText: "Cancelar",
        variant: "danger",
        ...options,
      },
      resolve,
    };
  });
}

function resolveActive(value: boolean) {
  const pending = active.value;
  if (!pending) return;
  active.value = null;
  pending.resolve(value);
}

export function useConfirm() {
  return { confirm, active, resolveActive };
}
