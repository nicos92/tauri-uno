import { computed, ref } from "vue";

export interface UsePaginationOptions {
  fetch: (limit: number, offset: number) => Promise<void>;
  getTotal: () => number;
  pageSizeOptions?: number[];
}

export const DEFAULT_PAGE_SIZE_OPTIONS = [5, 10, 15, 20, 25];

export function usePagination({
  fetch,
  getTotal,
  pageSizeOptions = DEFAULT_PAGE_SIZE_OPTIONS,
}: UsePaginationOptions) {
  const pageSize = ref(10);
  const offset = ref(0);

  const currentPage = computed(
    () => Math.floor(offset.value / pageSize.value) + 1,
  );

  const totalPages = computed(() =>
    Math.max(1, Math.ceil(getTotal() / pageSize.value)),
  );

  const pages = computed<Array<number | "...">>(() => {
    const total = totalPages.value;
    const current = currentPage.value;
    const window = 2;
    const result: Array<number | "..."> = [];
    for (let p = 1; p <= total; p++) {
      if (
        p === 1 ||
        p === total ||
        (p >= current - window && p <= current + window)
      ) {
        result.push(p);
      } else if (result[result.length - 1] !== "...") {
        result.push("...");
      }
    }
    return result;
  });

  async function goToPage(page: number | "...") {
    if (typeof page !== "number") return;
    if (page === currentPage.value || page < 1 || page > totalPages.value) return;
    offset.value = (page - 1) * pageSize.value;
    await fetch(pageSize.value, offset.value);
  }

  async function nextPage() {
    await goToPage(currentPage.value + 1);
  }

  async function prevPage() {
    await goToPage(currentPage.value - 1);
  }

  async function handlePageSizeChange() {
    offset.value = 0;
    await fetch(pageSize.value, offset.value);
  }

  async function refresh() {
    await fetch(pageSize.value, offset.value);
  }

  return {
    pageSize,
    pageSizeOptions,
    offset,
    currentPage,
    totalPages,
    pages,
    goToPage,
    nextPage,
    prevPage,
    handlePageSizeChange,
    refresh,
  };
}
