import { defineStore } from "pinia";
import { ref } from "vue";
import type {
  CreatePresupuestoRequest,
  PresupuestoEstado,
  PresupuestoWithDetalle,
} from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { presupuestoRepository } from "../../infrastructure/di";

export const usePresupuestosStore = defineStore("presupuestos", () => {
  const presupuestos = ref<PresupuestoWithDetalle[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const total = ref(0);
  const limit = ref(50);
  const offset = ref(0);

  async function crearPresupuesto(
    request: CreatePresupuestoRequest,
  ): Promise<PresupuestoWithDetalle | null> {
    loading.value = true;
    error.value = null;
    try {
      const presupuesto = await presupuestoRepository.crearPresupuesto(request);
      presupuestos.value.unshift(presupuesto);
      return presupuesto;
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function fetchPresupuestos(filters?: {
    limit?: number;
    offset?: number;
    estado?: PresupuestoEstado;
    fecha_desde?: string;
    fecha_hasta?: string;
    query?: string;
  }) {
    loading.value = true;
    error.value = null;
    try {
      const page = await presupuestoRepository.getAllPresupuestos({
        limit: filters?.limit ?? limit.value,
        offset: filters?.offset ?? offset.value,
        estado: filters?.estado,
        fecha_desde: filters?.fecha_desde,
        fecha_hasta: filters?.fecha_hasta,
        query: filters?.query,
      });
      presupuestos.value = page.items;
      total.value = page.total;
      limit.value = page.limit;
      offset.value = page.offset;
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function getPresupuestoById(
    id: number,
  ): Promise<PresupuestoWithDetalle | null> {
    error.value = null;
    try {
      return await presupuestoRepository.getPresupuestoById(id);
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  async function cambiarEstado(
    id: number,
    estado: PresupuestoEstado,
  ): Promise<boolean> {
    error.value = null;
    try {
      await presupuestoRepository.cambiarEstadoPresupuesto(id, estado);
      await fetchPresupuestos({ limit: limit.value, offset: offset.value });
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  return {
    presupuestos,
    loading,
    error,
    total,
    limit,
    offset,
    crearPresupuesto,
    fetchPresupuestos,
    getPresupuestoById,
    cambiarEstado,
  };
});
