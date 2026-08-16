import { defineStore } from "pinia";
import { ref } from "vue";
import type { CreateVentaRequest, VentaWithDetalle } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { VentasApiRepository } from "../../infrastructure/api/ventaRepository";
import { useStockStore } from "./stockStore";

const ventasRepository = new VentasApiRepository();

export const useVentasStore = defineStore("ventas", () => {
  const ventas = ref<VentaWithDetalle[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const diaCerrado = ref(false);
  const total = ref(0);
  const limit = ref(50);
  const offset = ref(0);

  async function checkDiaCerrado() {
    try {
      diaCerrado.value = await ventasRepository.isDiaCerrado();
    } catch {
      diaCerrado.value = false;
    }
  }

  async function fetchVentas(filters?: { limit?: number; offset?: number }) {
    loading.value = true;
    error.value = null;
    try {
      const page = await ventasRepository.getAllVentas({
        limit: filters?.limit ?? limit.value,
        offset: filters?.offset ?? offset.value,
      });
      ventas.value = page.items;
      total.value = page.total;
      limit.value = page.limit;
      offset.value = page.offset;
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function getVentaById(id: number): Promise<VentaWithDetalle | null> {
    try {
      return await ventasRepository.getVentaById(id);
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  async function getVentasPorCliente(
    clienteId: number,
  ): Promise<VentaWithDetalle[] | null> {
    try {
      return await ventasRepository.getVentasPorCliente(clienteId);
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  async function createVenta(
    request: CreateVentaRequest,
  ): Promise<VentaWithDetalle | null> {
    error.value = null;
    try {
      const venta = await ventasRepository.createVenta(request);
      await fetchVentas({ limit: limit.value, offset: offset.value });
      await useStockStore().fetchStock();
      return venta;
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  async function anularVenta(id: number): Promise<boolean> {
    error.value = null;
    try {
      await ventasRepository.anularVenta(id);
      await fetchVentas({ limit: limit.value, offset: offset.value });
      await useStockStore().fetchStock();
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  return {
    ventas,
    loading,
    error,
    diaCerrado,
    total,
    limit,
    offset,
    fetchVentas,
    getVentaById,
    getVentasPorCliente,
    createVenta,
    anularVenta,
    checkDiaCerrado,
  };
});
